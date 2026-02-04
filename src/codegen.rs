use crate::ast::*;
use crate::config::Config;
use crate::scratch::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CodeGenerator {
    /// Counter for generating unique block IDs
    block_counter: u64,
    /// Map of variable names to their IDs
    variables: HashMap<String, String>,
    /// Map of list names to their IDs
    lists: HashMap<String, String>,
    /// Map of broadcast names to their IDs
    broadcasts: HashMap<String, String>,
    /// Map of matrix names to their widths (number of columns)
    matrix_widths: HashMap<String, usize>,
    /// Current Y position for placing top-level blocks
    current_y: f64,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            block_counter: 0,
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcasts: HashMap::new(),
            matrix_widths: HashMap::new(),
            current_y: 0.0,
        }
    }

    fn generate_id(&mut self) -> String {
        let id = Uuid::new_v4().to_string().replace("-", "")[..20].to_string();
        self.block_counter += 1;
        id
    }

    fn get_or_create_variable(&mut self, name: &str) -> String {
        if let Some(id) = self.variables.get(name) {
            id.clone()
        } else {
            let id = self.generate_id();
            self.variables.insert(name.to_string(), id.clone());
            id
        }
    }

    fn get_or_create_list(&mut self, name: &str) -> String {
        if let Some(id) = self.lists.get(name) {
            id.clone()
        } else {
            let id = self.generate_id();
            self.lists.insert(name.to_string(), id.clone());
            id
        }
    }

    fn get_or_create_broadcast(&mut self, name: &str) -> String {
        if let Some(id) = self.broadcasts.get(name) {
            id.clone()
        } else {
            let id = self.generate_id();
            self.broadcasts.insert(name.to_string(), id.clone());
            id
        }
    }

    pub fn generate(&mut self, _config: &Config, program: &Program) -> ScratchProject {
        let mut project = ScratchProject::new();

        // Add extensions
        for extension in &program.extensions {
            project.extensions.push(extension.clone());
        }

        // First pass: collect all variables and broadcasts
        for global in &program.globals {
            self.get_or_create_variable(&global.name);
        }
        self.collect_broadcasts(program);

        // Create stage
        let stage = if let Some(stage_node) = &program.stage {
            self.generate_stage(stage_node, &program.globals)
        } else {
            let mut stage = Target::new_stage();
            self.add_global_variables(&mut stage, &program.globals);
            stage
        };
        project.targets.push(stage);

        // Create sprites
        for (i, sprite_node) in program.sprites.iter().enumerate() {
            let sprite = self.generate_sprite(sprite_node, i + 1, &program.globals);
            project.targets.push(sprite);
        }

        // Add broadcasts to stage
        if let Some(stage) = project.targets.first_mut() {
            for (name, id) in &self.broadcasts {
                stage.broadcasts.insert(id.clone(), name.clone());
            }
        }

        // Add monitors for global variables
        self.add_variable_monitors(&mut project, &program.globals);

        project
    }

    fn add_variable_monitors(&mut self, project: &mut ScratchProject, globals: &[GlobalVar]) {
        let mut y = 5.0;
        for global in globals {
            let var_id = self.variables.get(&global.name).unwrap();
            let value = self.expr_to_json_value(&global.initial_value);

            // Use monitor configuration if provided, otherwise use defaults
            let x = global.monitor_x.unwrap_or(5.0);
            let monitor_y = global.monitor_y.unwrap_or(y);
            let visible = global.monitor_visible.unwrap_or(false);

            let mut monitor = Monitor::variable(var_id, &global.name, value, x, monitor_y);
            monitor.visible = visible;
            project.monitors.push(monitor);

            // Only auto-increment y if not explicitly set
            if global.monitor_y.is_none() {
                y += 40.0;
            }
        }
    }

    fn collect_broadcasts(&mut self, program: &Program) {
        // Collect from stage
        if let Some(stage) = &program.stage {
            if let Some(code) = &stage.code {
                self.collect_broadcasts_from_code(code);
            }
        }

        // Collect from sprites
        for sprite in &program.sprites {
            if let Some(code) = &sprite.code {
                self.collect_broadcasts_from_code(code);
            }
        }
    }

    fn collect_broadcasts_from_code(&mut self, code: &CodeBlock) {
        for handler in &code.event_handlers {
            if let Event::Broadcast(name) = &handler.event {
                self.get_or_create_broadcast(name);
            }
            self.collect_broadcasts_from_statements(&handler.body);
        }
    }

    fn collect_broadcasts_from_statements(&mut self, statements: &[Statement]) {
        for stmt in statements {
            match stmt {
                Statement::BlockCall(call) => {
                    if call.category == "events"
                        && (call.block_name == "Broadcast" || call.block_name == "BroadcastAndWait")
                    {
                        if let Some(Expression::StringLiteral(name)) = call.args.first() {
                            self.get_or_create_broadcast(name);
                        }
                    }
                }
                Statement::Forever { body } => self.collect_broadcasts_from_statements(body),
                Statement::Repeat { body, .. } => self.collect_broadcasts_from_statements(body),
                Statement::RepeatUntil { body, .. } => {
                    self.collect_broadcasts_from_statements(body);
                }
                Statement::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    self.collect_broadcasts_from_statements(then_body);
                    if let Some(else_b) = else_body {
                        self.collect_broadcasts_from_statements(else_b);
                    }
                }
                _ => {}
            }
        }
    }

    fn add_global_variables(&mut self, target: &mut Target, globals: &[GlobalVar]) {
        for global in globals {
            match &global.var_type {
                VarType::Matrix(_) => {
                    // Handle matrix types - store width for indexing
                    let list_id = self.get_or_create_list(&global.name);
                    let initial_items = self.expr_to_list_items(&global.initial_value);

                    // Extract matrix width from the literal
                    if let Expression::MatrixLiteral(rows) = &global.initial_value {
                        if let Some(first_row) = rows.first() {
                            self.matrix_widths
                                .insert(global.name.clone(), first_row.len());
                        }
                    }

                    target
                        .lists
                        .insert(list_id, List::new(&global.name, initial_items));
                }
                VarType::List(_) => {
                    // Handle list types
                    let list_id = self.get_or_create_list(&global.name);
                    let initial_items = self.expr_to_list_items(&global.initial_value);
                    target
                        .lists
                        .insert(list_id, List::new(&global.name, initial_items));
                }
                _ => {
                    // Handle scalar types
                    let var_id = self.get_or_create_variable(&global.name);
                    let initial_value = self.expr_to_json_value(&global.initial_value);
                    target
                        .variables
                        .insert(var_id, Variable::new(&global.name, initial_value));
                }
            }
        }
    }

    fn expr_to_json_value(&self, expr: &Expression) -> serde_json::Value {
        match expr {
            Expression::IntLiteral(n) => serde_json::json!(n),
            Expression::FloatLiteral(n) => serde_json::json!(n),
            Expression::StringLiteral(s) => serde_json::json!(s),
            Expression::BoolLiteral(b) => serde_json::json!(if *b { 1 } else { 0 }),
            _ => serde_json::json!(0),
        }
    }

    fn expr_to_list_items(&self, expr: &Expression) -> Vec<serde_json::Value> {
        match expr {
            Expression::ListLiteral(items) => {
                items.iter().map(|e| self.expr_to_json_value(e)).collect()
            }
            Expression::MatrixLiteral(rows) => {
                // Flatten matrix into a single list for Scratch
                // Store as: [row0_col0, row0_col1, ..., row1_col0, row1_col1, ...]
                rows.iter()
                    .flat_map(|row| row.iter().map(|e| self.expr_to_json_value(e)))
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    fn generate_stage(&mut self, stage_node: &StageNode, globals: &[GlobalVar]) -> Target {
        let mut stage = Target::new_stage();
        self.add_global_variables(&mut stage, globals);
        self.current_y = 0.0;

        if let Some(code) = &stage_node.code {
            self.generate_code_block(&mut stage.blocks, code);
        }

        stage
    }

    fn generate_sprite(
        &mut self,
        sprite_node: &SpriteNode,
        layer_order: usize,
        globals: &[GlobalVar],
    ) -> Target {
        let mut sprite = Target::new_sprite(&sprite_node.name, layer_order);
        self.add_global_variables(&mut sprite, globals);
        self.current_y = 0.0;

        if let Some((x, y)) = sprite_node.position {
            sprite.x = Some(x);
            sprite.y = Some(y);
        }

        if let Some(size) = sprite_node.size {
            sprite.size = Some(size);
        }

        if let Some(code) = &sprite_node.code {
            self.generate_code_block(&mut sprite.blocks, code);
        }

        sprite
    }

    fn generate_code_block(&mut self, blocks: &mut HashMap<String, Block>, code: &CodeBlock) {
        // Generate event handlers
        for handler in &code.event_handlers {
            self.generate_event_handler(blocks, handler);
            self.current_y += 200.0;
        }

        // Generate custom functions (procedures)
        for function in &code.functions {
            self.generate_function(blocks, function);
            self.current_y += 200.0;
        }
    }

    fn generate_event_handler(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        handler: &EventHandler,
    ) {
        let hat_id = self.generate_id();
        let mut hat_block = self.create_hat_block(&handler.event);
        hat_block.top_level = true;
        hat_block.x = Some(0.0);
        hat_block.y = Some(self.current_y);

        // Generate body statements
        let first_stmt_id = self.generate_statements(blocks, &handler.body, Some(&hat_id));
        hat_block.next = first_stmt_id;

        blocks.insert(hat_id, hat_block);
    }

    fn create_hat_block(&mut self, event: &Event) -> Block {
        match event {
            Event::GreenFlag => Block::new("event_whenflagclicked"),
            Event::KeyPressed(key) => {
                let mut block = Block::new("event_whenkeypressed");
                block
                    .fields
                    .insert("KEY_OPTION".to_string(), Field::new(key, None));
                block
            }
            Event::Clicked => Block::new("event_whenthisspriteclicked"),
            Event::Broadcast(name) => {
                let mut block = Block::new("event_whenbroadcastreceived");
                let broadcast_id = self.broadcasts.get(name).cloned().unwrap_or_default();
                block.fields.insert(
                    "BROADCAST_OPTION".to_string(),
                    Field::new(name, Some(&broadcast_id)),
                );
                block
            }
            Event::BackdropSwitch(name) => {
                let mut block = Block::new("event_whenbackdropswitchesto");
                block
                    .fields
                    .insert("BACKDROP".to_string(), Field::new(name, None));
                block
            }
            Event::CloneStart => Block::new("control_start_as_clone"),
        }
    }

    fn generate_function(&mut self, blocks: &mut HashMap<String, Block>, function: &Function) {
        let define_id = self.generate_id();
        let proto_id = self.generate_id();

        // Build proccode (e.g., "myFunc %s %s")
        let mut proccode = function.name.clone();
        let mut argument_ids: Vec<String> = Vec::new();
        let mut argument_names: Vec<String> = Vec::new();
        let mut argument_defaults: Vec<String> = Vec::new();

        for param in &function.params {
            let arg_id = self.generate_id();
            argument_ids.push(arg_id);
            argument_names.push(param.name.clone());
            match param.param_type {
                VarType::Bool => {
                    proccode.push_str(" %b");
                    argument_defaults.push("false".to_string());
                }
                _ => {
                    proccode.push_str(" %s");
                    argument_defaults.push("".to_string());
                }
            }
        }

        // Create prototype block
        let mut proto_block = Block::new("procedures_prototype");
        proto_block.shadow = true;
        proto_block.parent = Some(define_id.clone());
        proto_block.mutation = Some(Mutation {
            tag_name: "mutation".to_string(),
            children: vec![],
            proccode: Some(proccode.clone()),
            argumentids: Some(serde_json::to_string(&argument_ids).unwrap()),
            argumentnames: Some(serde_json::to_string(&argument_names).unwrap()),
            argumentdefaults: Some(serde_json::to_string(&argument_defaults).unwrap()),
            warp: Some(if function.warp { "true" } else { "false" }.to_string()),
            hasnext: None,
        });

        // Create definition block
        let mut define_block = Block::new("procedures_definition");
        define_block.top_level = true;
        define_block.x = Some(0.0);
        define_block.y = Some(self.current_y);
        define_block
            .inputs
            .insert("custom_block".to_string(), Input::block(&proto_id));

        // Generate body
        let first_stmt_id = self.generate_statements(blocks, &function.body, Some(&define_id));
        define_block.next = first_stmt_id;

        blocks.insert(proto_id, proto_block);
        blocks.insert(define_id, define_block);
    }

    fn generate_statements(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        statements: &[Statement],
        parent_id: Option<&str>,
    ) -> Option<String> {
        if statements.is_empty() {
            return None;
        }

        let mut first_id: Option<String> = None;
        let mut prev_id: Option<String> = None;

        for stmt in statements {
            let stmt_id = self.generate_statement(blocks, stmt, parent_id, prev_id.as_deref());

            if let Some(id) = &stmt_id {
                if first_id.is_none() {
                    first_id = Some(id.clone());
                }

                // Link previous block to this one
                if let Some(prev) = &prev_id {
                    if let Some(prev_block) = blocks.get_mut(prev) {
                        prev_block.next = Some(id.clone());
                    }
                }

                prev_id = Some(id.clone());
            }
        }

        first_id
    }

    fn generate_statement(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        stmt: &Statement,
        parent_id: Option<&str>,
        _prev_id: Option<&str>,
    ) -> Option<String> {
        match stmt {
            Statement::BlockCall(call) => {
                let block_id = self.generate_id();
                let block = self.generate_block_call(blocks, call, parent_id, &block_id);
                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::SetVariable { name, value } => {
                let block_id = self.generate_id();
                let var_id = self.get_or_create_variable(name);
                let mut block = Block::new("data_setvariableto");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("VARIABLE".to_string(), Field::new(name, Some(&var_id)));

                let value_input = self.generate_input(blocks, value, &block_id);
                block.inputs.insert("VALUE".to_string(), value_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::ChangeVariable { name, value, op } => {
                let block_id = self.generate_id();
                let var_id = self.get_or_create_variable(name);

                match op {
                    ChangeOp::Add => {
                        // Use native Scratch "change variable by" block
                        let mut block = Block::new("data_changevariableby");
                        block.parent = parent_id.map(|s| s.to_string());
                        block
                            .fields
                            .insert("VARIABLE".to_string(), Field::new(name, Some(&var_id)));

                        let value_input = self.generate_input(blocks, value, &block_id);
                        block.inputs.insert("VALUE".to_string(), value_input);

                        blocks.insert(block_id.clone(), block);
                        Some(block_id)
                    }
                    _ => {
                        // For other operations, generate: set variable to (variable OP value)
                        let mut block = Block::new("data_setvariableto");
                        block.parent = parent_id.map(|s| s.to_string());
                        block
                            .fields
                            .insert("VARIABLE".to_string(), Field::new(name, Some(&var_id)));

                        // Create the operator block
                        let op_block_id = self.generate_id();
                        let opcode = match op {
                            ChangeOp::Sub => "operator_subtract",
                            ChangeOp::Mul => "operator_multiply",
                            ChangeOp::Div => "operator_divide",
                            ChangeOp::Pow => "operator_mathop",
                            ChangeOp::Add => unreachable!(),
                        };

                        if *op == ChangeOp::Pow {
                            // Power uses mathop with "e ^" and "ln" to compute: e^(ln(base) * exponent) = base^exponent

                            // Create ln(var) block
                            let ln_block_id = self.generate_id();
                            let mut ln_block = Block::new("operator_mathop");
                            ln_block.parent = Some(op_block_id.clone());
                            ln_block
                                .fields
                                .insert("OPERATOR".to_string(), Field::new("ln", None));
                            // Input is the variable
                            let var_input = Input::variable(name, &var_id);
                            ln_block.inputs.insert("NUM".to_string(), var_input);
                            blocks.insert(ln_block_id.clone(), ln_block);

                            // Create multiply block: ln(var) * value
                            let mul_block_id = self.generate_id();
                            let mut mul_block = Block::new("operator_multiply");
                            mul_block.parent = Some(op_block_id.clone());
                            mul_block
                                .inputs
                                .insert("NUM1".to_string(), Input::block(&ln_block_id));
                            // Update ln_block parent
                            if let Some(ln_b) = blocks.get_mut(&ln_block_id) {
                                ln_b.parent = Some(mul_block_id.clone());
                            }
                            let value_input = self.generate_input(blocks, value, &mul_block_id);
                            mul_block.inputs.insert("NUM2".to_string(), value_input);
                            blocks.insert(mul_block_id.clone(), mul_block);

                            // Create e^ block: e^(ln(var) * value)
                            let mut exp_block = Block::new("operator_mathop");
                            exp_block.parent = Some(block_id.clone());
                            exp_block
                                .fields
                                .insert("OPERATOR".to_string(), Field::new("e ^", None));
                            exp_block
                                .inputs
                                .insert("NUM".to_string(), Input::block(&mul_block_id));
                            // Update mul_block parent
                            if let Some(mul_b) = blocks.get_mut(&mul_block_id) {
                                mul_b.parent = Some(op_block_id.clone());
                            }
                            blocks.insert(op_block_id.clone(), exp_block);

                            block
                                .inputs
                                .insert("VALUE".to_string(), Input::block(&op_block_id));
                        } else {
                            // For sub, mul, div - create a simple operator block
                            let mut op_block = Block::new(opcode);
                            op_block.parent = Some(block_id.clone());

                            // NUM1 is the current variable value
                            let var_input = Input::variable(name, &var_id);
                            op_block.inputs.insert("NUM1".to_string(), var_input);

                            // NUM2 is the value to operate with
                            let value_input = self.generate_input(blocks, value, &op_block_id);
                            op_block.inputs.insert("NUM2".to_string(), value_input);

                            blocks.insert(op_block_id.clone(), op_block);

                            block
                                .inputs
                                .insert("VALUE".to_string(), Input::block(&op_block_id));
                        }

                        blocks.insert(block_id.clone(), block);
                        Some(block_id)
                    }
                }
            }

            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let block_id = self.generate_id();
                let opcode = if else_body.is_some() {
                    "control_if_else"
                } else {
                    "control_if"
                };
                let mut block = Block::new(opcode);
                block.parent = parent_id.map(|s| s.to_string());

                // Condition
                let cond_input = self.generate_condition_input(blocks, condition, &block_id);
                block.inputs.insert("CONDITION".to_string(), cond_input);

                // Then body
                let then_first = self.generate_statements(blocks, then_body, Some(&block_id));
                block.inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::substack(then_first.as_deref()),
                );

                // Else body
                if let Some(else_b) = else_body {
                    let else_first = self.generate_statements(blocks, else_b, Some(&block_id));
                    block.inputs.insert(
                        "SUBSTACK2".to_string(),
                        Input::substack(else_first.as_deref()),
                    );
                }

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::Forever { body } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_forever");
                block.parent = parent_id.map(|s| s.to_string());

                let body_first = self.generate_statements(blocks, body, Some(&block_id));
                block.inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::substack(body_first.as_deref()),
                );

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::Repeat { times, body } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_repeat");
                block.parent = parent_id.map(|s| s.to_string());

                let times_input = self.generate_input(blocks, times, &block_id);
                block.inputs.insert("TIMES".to_string(), times_input);

                let body_first = self.generate_statements(blocks, body, Some(&block_id));
                block.inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::substack(body_first.as_deref()),
                );

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::RepeatUntil { condition, body } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_repeat_until");
                block.parent = parent_id.map(|s| s.to_string());

                let cond_input = self.generate_condition_input(blocks, condition, &block_id);
                block.inputs.insert("CONDITION".to_string(), cond_input);

                let body_first = self.generate_statements(blocks, body, Some(&block_id));
                block.inputs.insert(
                    "SUBSTACK".to_string(),
                    Input::substack(body_first.as_deref()),
                );

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::Wait { duration } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_wait");
                block.parent = parent_id.map(|s| s.to_string());

                let duration_expr = self.unwrap_unit_value(duration);
                let dur_input = self.generate_input(blocks, duration_expr, &block_id);
                block.inputs.insert("DURATION".to_string(), dur_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::Stop { mode } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_stop");
                block.parent = parent_id.map(|s| s.to_string());

                let stop_option = match mode.as_str() {
                    "all" => "all",
                    "this script" => "this script",
                    "other scripts in sprite" => "other scripts in sprite",
                    _ => "all",
                };
                block
                    .fields
                    .insert("STOP_OPTION".to_string(), Field::new(stop_option, None));
                block.mutation = Some(Mutation {
                    tag_name: "mutation".to_string(),
                    children: vec![],
                    proccode: None,
                    argumentids: None,
                    argumentnames: None,
                    argumentdefaults: None,
                    warp: None,
                    hasnext: Some(mode != "all" && mode != "this script"),
                });

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::CreateClone { target } => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_create_clone_of");
                block.parent = parent_id.map(|s| s.to_string());

                // Create the menu block
                let menu_id = self.generate_id();
                let mut menu_block = Block::new("control_create_clone_of_menu");
                menu_block.parent = Some(block_id.clone());
                menu_block.shadow = true;
                menu_block
                    .fields
                    .insert("CLONE_OPTION".to_string(), Field::new(target, None));
                blocks.insert(menu_id.clone(), menu_block);

                block
                    .inputs
                    .insert("CLONE_OPTION".to_string(), Input::block(&menu_id));

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::DeleteClone => {
                let block_id = self.generate_id();
                let mut block = Block::new("control_delete_this_clone");
                block.parent = parent_id.map(|s| s.to_string());
                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::FunctionCall { name, args } => {
                let block_id = self.generate_id();
                let mut block = Block::new("procedures_call");
                block.parent = parent_id.map(|s| s.to_string());

                // Build proccode
                let mut proccode = name.clone();
                let mut argument_ids: Vec<String> = Vec::new();

                for arg in args.iter() {
                    let arg_id = self.generate_id();
                    argument_ids.push(arg_id.clone());
                    proccode.push_str(" %s");

                    let input = self.generate_input(blocks, arg, &block_id);
                    block.inputs.insert(arg_id, input);
                }

                // Note: The warp setting in procedure calls doesn't affect execution;
                // it's determined by the procedure definition. We set it to "false" here
                // as it's ignored for calls anyway.
                block.mutation = Some(Mutation {
                    tag_name: "mutation".to_string(),
                    children: vec![],
                    proccode: Some(proccode),
                    argumentids: Some(serde_json::to_string(&argument_ids).unwrap()),
                    argumentnames: None,
                    argumentdefaults: None,
                    warp: Some("false".to_string()),
                    hasnext: None,
                });

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            // List operations
            Statement::ListAdd { list, value } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_addtolist");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                let value_input = self.generate_input(blocks, value, &block_id);
                block.inputs.insert("ITEM".to_string(), value_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::ListDelete { list, index } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_deleteoflist");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                let index_input = self.generate_list_index_input(blocks, index, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::ListInsert { list, index, value } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_insertatlist");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                // Convert 0-based index to 1-based for Scratch
                let index_plus_one = Expression::BinaryOp {
                    left: Box::new(index.clone()),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                let index_input = self.generate_input(blocks, &index_plus_one, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                let value_input = self.generate_input(blocks, value, &block_id);
                block.inputs.insert("ITEM".to_string(), value_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::ListReplace { list, index, value } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_replaceitemoflist");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                // Convert 0-based index to 1-based for Scratch
                let index_plus_one = Expression::BinaryOp {
                    left: Box::new(index.clone()),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                let index_input = self.generate_input(blocks, &index_plus_one, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                let value_input = self.generate_input(blocks, value, &block_id);
                block.inputs.insert("ITEM".to_string(), value_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }

            Statement::MatrixReplace {
                matrix,
                x,
                y,
                value,
            } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(matrix);
                let mut block = Block::new("data_replaceitemoflist");
                block.parent = parent_id.map(|s| s.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(matrix, Some(&list_id)));

                // For matrix stored as flattened list, calculate: x + y * width + 1
                let width = self.matrix_widths.get(matrix).copied().unwrap_or(1) as i64;
                let index_expr = Expression::BinaryOp {
                    left: Box::new(Expression::BinaryOp {
                        left: Box::new(x.clone()),
                        op: BinaryOperator::Add,
                        right: Box::new(Expression::BinaryOp {
                            left: Box::new(y.clone()),
                            op: BinaryOperator::Mul,
                            right: Box::new(Expression::IntLiteral(width)),
                        }),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                let index_input = self.generate_input(blocks, &index_expr, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                let value_input = self.generate_input(blocks, value, &block_id);
                block.inputs.insert("ITEM".to_string(), value_input);

                blocks.insert(block_id.clone(), block);
                Some(block_id)
            }
        }
    }

    fn generate_list_index_input(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        index: &ListIndex,
        parent_id: &str,
    ) -> Input {
        match index {
            ListIndex::Index(expr) => {
                // Convert 0-based index to 1-based for Scratch
                let index_plus_one = Expression::BinaryOp {
                    left: Box::new(expr.clone()),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                self.generate_input(blocks, &index_plus_one, parent_id)
            }
            ListIndex::All => Input::literal(input_types::STRING, serde_json::json!("all")),
            ListIndex::Last => Input::literal(input_types::STRING, serde_json::json!("last")),
            ListIndex::Random => Input::literal(input_types::STRING, serde_json::json!("random")),
        }
    }

    fn generate_block_call(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        call: &BlockCall,
        parent_id: Option<&str>,
        block_id: &str,
    ) -> Block {
        let opcode = self.get_opcode(&call.category, &call.block_name);
        let mut block = Block::new(&opcode);
        block.parent = parent_id.map(|s| s.to_string());

        // Add inputs based on block type
        self.add_block_inputs(blocks, &mut block, call, block_id);

        block
    }

    fn get_opcode(&self, category: &str, block_name: &str) -> String {
        (match (category, block_name) {
            // Motion
            ("motion", "Move") => "motion_movesteps",
            ("motion", "TurnRight") => "motion_turnright",
            ("motion", "TurnLeft") => "motion_turnleft",
            ("motion", "GoTo") => "motion_goto",
            ("motion", "GoToXY") => "motion_gotoxy",
            ("motion", "GlideTo") => "motion_glideto",
            ("motion", "GlideToXY") => "motion_glidesecstoxy",
            ("motion", "PointInDirection") => "motion_pointindirection",
            ("motion", "PointTowards") => "motion_pointtowards",
            ("motion", "ChangeX") => "motion_changexby",
            ("motion", "ChangeY") => "motion_changeyby",
            ("motion", "SetX") => "motion_setx",
            ("motion", "SetY") => "motion_sety",
            ("motion", "IfOnEdgeBounce") => "motion_ifonedgebounce",
            ("motion", "SetRotationStyle") => "motion_setrotationstyle",

            // Looks
            ("looks", "Say") => "looks_say",
            ("looks", "SayTimed") => "looks_sayforsecs",
            ("looks", "Think") => "looks_think",
            ("looks", "ThinkTimed") => "looks_thinkforsecs",
            ("looks", "SwitchCostume") => "looks_switchcostumeto",
            ("looks", "NextCostume") => "looks_nextcostume",
            ("looks", "SwitchBackdrop") => "looks_switchbackdropto",
            ("looks", "NextBackdrop") => "looks_nextbackdrop",
            ("looks", "ChangeSize") => "looks_changesizeby",
            ("looks", "SetSize") => "looks_setsizeto",
            ("looks", "ChangeEffect") => "looks_changeeffectby",
            ("looks", "SetEffect") => "looks_seteffectto",
            ("looks", "ClearEffects") => "looks_cleargraphiceffects",
            ("looks", "Show") => "looks_show",
            ("looks", "Hide") => "looks_hide",
            ("looks", "GoToLayer") => "looks_gotofrontback",
            ("looks", "ChangeLayer") => "looks_goforwardbackwardlayers",

            // Sound
            ("sound", "Play") => "sound_play",
            ("sound", "PlayUntilDone") => "sound_playuntildone",
            ("sound", "StopAllSounds") => "sound_stopallsounds",
            ("sound", "ChangeVolume") => "sound_changevolumeby",
            ("sound", "SetVolume") => "sound_setvolumeto",

            // Events
            ("events", "Broadcast") => "event_broadcast",
            ("events", "BroadcastAndWait") => "event_broadcastandwait",

            // Sensing
            ("sensing", "AskAndWait") => "sensing_askandwait",
            ("sensing", "ResetTimer") => "sensing_resettimer",

            // Pen
            ("pen", "Clear") => "pen_clear",
            ("pen", "Stamp") => "pen_stamp",
            ("pen", "PenDown") => "pen_penDown",
            ("pen", "PenUp") => "pen_penUp",
            ("pen", "SetPenColor") => "pen_setPenColorToColor",
            ("pen", "ChangePenColor") => "pen_changePenColorParamBy",
            ("pen", "SetPenColorParam") => "pen_setPenColorParamTo",
            ("pen", "ChangePenSize") => "pen_changePenSizeBy",
            ("pen", "SetPenSize") => "pen_setPenSizeTo",

            _ => {
                return format!("{}_{}", category, block_name.to_lowercase());
            }
        })
        .to_string()
    }

    fn add_block_inputs(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        block: &mut Block,
        call: &BlockCall,
        block_id: &str,
    ) {
        match (call.category.as_str(), call.block_name.as_str()) {
            // Motion blocks
            ("motion", "Move") => {
                if let Some(steps) = call.args.first() {
                    let input = self.generate_input(blocks, steps, block_id);
                    block.inputs.insert("STEPS".to_string(), input);
                }
            }
            ("motion", "TurnRight") | ("motion", "TurnLeft") => {
                if let Some(degrees) = call.args.first() {
                    let input = self.generate_input(blocks, degrees, block_id);
                    block.inputs.insert("DEGREES".to_string(), input);
                }
            }
            ("motion", "GoToXY") => {
                if let Some(x) = call.args.first() {
                    let input = self.generate_input(blocks, x, block_id);
                    block.inputs.insert("X".to_string(), input);
                }
                if let Some(y) = call.args.get(1) {
                    let input = self.generate_input(blocks, y, block_id);
                    block.inputs.insert("Y".to_string(), input);
                }
            }
            ("motion", "ChangeX") => {
                if let Some(x) = call.args.first() {
                    let input = self.generate_input(blocks, x, block_id);
                    block.inputs.insert("DX".to_string(), input);
                }
            }
            ("motion", "SetX") => {
                if let Some(x) = call.args.first() {
                    let input = self.generate_input(blocks, x, block_id);
                    block.inputs.insert("X".to_string(), input);
                }
            }
            ("motion", "ChangeY") => {
                if let Some(y) = call.args.first() {
                    let input = self.generate_input(blocks, y, block_id);
                    block.inputs.insert("DY".to_string(), input);
                }
            }
            ("motion", "SetY") => {
                if let Some(y) = call.args.first() {
                    let input = self.generate_input(blocks, y, block_id);
                    block.inputs.insert("Y".to_string(), input);
                }
            }
            ("motion", "PointInDirection") => {
                if let Some(dir) = call.args.first() {
                    let input = self.generate_input(blocks, dir, block_id);
                    block.inputs.insert("DIRECTION".to_string(), input);
                }
            }
            ("motion", "GlideToXY") => {
                if let Some(secs) = call.args.first() {
                    let secs_expr = self.unwrap_unit_value(secs);
                    let input = self.generate_input(blocks, secs_expr, block_id);
                    block.inputs.insert("SECS".to_string(), input);
                }
                if let Some(x) = call.args.get(1) {
                    let input = self.generate_input(blocks, x, block_id);
                    block.inputs.insert("X".to_string(), input);
                }
                if let Some(y) = call.args.get(2) {
                    let input = self.generate_input(blocks, y, block_id);
                    block.inputs.insert("Y".to_string(), input);
                }
            }

            // Looks blocks
            ("looks", "Say") | ("looks", "Think") => {
                if let Some(msg) = call.args.first() {
                    let input = self.generate_input(blocks, msg, block_id);
                    block.inputs.insert("MESSAGE".to_string(), input);
                }
            }
            ("looks", "SayTimed") | ("looks", "ThinkTimed") => {
                if let Some(msg) = call.args.first() {
                    let input = self.generate_input(blocks, msg, block_id);
                    block.inputs.insert("MESSAGE".to_string(), input);
                }
                if let Some(secs) = call.args.get(1) {
                    let secs_expr = self.unwrap_unit_value(secs);
                    let input = self.generate_input(blocks, secs_expr, block_id);
                    block.inputs.insert("SECS".to_string(), input);
                }
            }
            ("looks", "ChangeSize") => {
                if let Some(change) = call.args.first() {
                    let input = self.generate_input(blocks, change, block_id);
                    block.inputs.insert("CHANGE".to_string(), input);
                }
            }
            ("looks", "SetSize") => {
                if let Some(size) = call.args.first() {
                    let input = self.generate_input(blocks, size, block_id);
                    block.inputs.insert("SIZE".to_string(), input);
                }
            }

            // Events blocks
            ("events", "Broadcast") | ("events", "BroadcastAndWait") => {
                if let Some(Expression::StringLiteral(msg)) = call.args.first() {
                    let broadcast_id = self.get_or_create_broadcast(msg);
                    block.inputs.insert(
                        "BROADCAST_INPUT".to_string(),
                        Input::literal(
                            input_types::BROADCAST,
                            serde_json::json!([msg, broadcast_id]),
                        ),
                    );
                }
            }

            // Sensing blocks
            ("sensing", "AskAndWait") => {
                if let Some(question) = call.args.first() {
                    let input = self.generate_input(blocks, question, block_id);
                    block.inputs.insert("QUESTION".to_string(), input);
                }
            }

            // Pen blocks
            ("pen", "SetPenColor") => {
                if let Some(color) = call.args.first() {
                    let input = self.generate_input(blocks, color, block_id);
                    block.inputs.insert("COLOR".to_string(), input);
                }
            }
            ("pen", "ChangePenColor") => {
                if let Some(change) = call.args.first() {
                    let input = self.generate_input(blocks, change, block_id);
                    block.inputs.insert("COLOR_PARAM".to_string(), input);
                }
            }
            ("pen", "SetPenColorParam") => {
                if let Some(param) = call.args.first() {
                    let input = self.generate_input(blocks, param, block_id);
                    block.inputs.insert("COLOR_PARAM".to_string(), input);
                }
                if let Some(value) = call.args.get(1) {
                    let input = self.generate_input(blocks, value, block_id);
                    block.inputs.insert("VALUE".to_string(), input);
                }
            }
            ("pen", "ChangePenSize") => {
                if let Some(change) = call.args.first() {
                    let input = self.generate_input(blocks, change, block_id);
                    block.inputs.insert("SIZE".to_string(), input);
                }
            }
            ("pen", "SetPenSize") => {
                if let Some(size) = call.args.first() {
                    let input = self.generate_input(blocks, size, block_id);
                    block.inputs.insert("SIZE".to_string(), input);
                }
            }

            _ => {}
        }
    }

    fn generate_input(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        expr: &Expression,
        parent_id: &str,
    ) -> Input {
        match expr {
            Expression::IntLiteral(n) => {
                Input::literal(input_types::NUMBER, serde_json::json!(n.to_string()))
            }
            Expression::FloatLiteral(n) => {
                Input::literal(input_types::NUMBER, serde_json::json!(n.to_string()))
            }
            Expression::StringLiteral(s) => {
                Input::literal(input_types::STRING, serde_json::json!(s))
            }
            Expression::BoolLiteral(b) => Input::literal(
                input_types::STRING,
                serde_json::json!(if *b { "true" } else { "false" }),
            ),
            Expression::Variable(name) => {
                let var_id = self.get_or_create_variable(name);
                Input::variable(name, &var_id)
            }
            Expression::UnitValue { value, .. } => self.generate_input(blocks, value, parent_id),
            Expression::BinaryOp { left, op, right } => {
                // Special handling for power operations
                if op == &BinaryOperator::Power {
                    return self.generate_power_operation(blocks, left, right, parent_id);
                }

                let op_block_id = self.generate_id();
                let opcode = match op {
                    BinaryOperator::Add => "operator_add",
                    BinaryOperator::Sub => "operator_subtract",
                    BinaryOperator::Mul => "operator_multiply",
                    BinaryOperator::Div => "operator_divide",
                    BinaryOperator::Mod => "operator_mod",
                    BinaryOperator::Lt => "operator_lt",
                    BinaryOperator::Gt => "operator_gt",
                    BinaryOperator::Eq => "operator_equals",
                    BinaryOperator::And => "operator_and",
                    BinaryOperator::Or => "operator_or",
                    _ => "operator_equals",
                };

                let mut op_block = Block::new(opcode);
                op_block.parent = Some(parent_id.to_string());

                let (left_key, right_key) = match op {
                    BinaryOperator::And | BinaryOperator::Or => ("OPERAND1", "OPERAND2"),
                    _ => ("NUM1", "NUM2"),
                };

                let left_input = self.generate_input(blocks, left, &op_block_id);
                let right_input = self.generate_input(blocks, right, &op_block_id);

                op_block.inputs.insert(left_key.to_string(), left_input);
                op_block.inputs.insert(right_key.to_string(), right_input);

                blocks.insert(op_block_id.clone(), op_block);
                Input::block(&op_block_id)
            }
            Expression::UnaryOp { op, operand } => {
                match op {
                    UnaryOperator::Not => {
                        let not_block_id = self.generate_id();
                        let mut not_block = Block::new("operator_not");
                        not_block.parent = Some(parent_id.to_string());

                        let operand_input =
                            self.generate_condition_input(blocks, operand, &not_block_id);
                        not_block
                            .inputs
                            .insert("OPERAND".to_string(), operand_input);

                        blocks.insert(not_block_id.clone(), not_block);
                        Input::block(&not_block_id)
                    }
                    UnaryOperator::Neg => {
                        // Represent -x as (0 - x)
                        let sub_block_id = self.generate_id();
                        let mut sub_block = Block::new("operator_subtract");
                        sub_block.parent = Some(parent_id.to_string());

                        sub_block.inputs.insert(
                            "NUM1".to_string(),
                            Input::literal(input_types::NUMBER, serde_json::json!(0)),
                        );

                        let operand_input = self.generate_input(blocks, operand, &sub_block_id);
                        sub_block.inputs.insert("NUM2".to_string(), operand_input);

                        blocks.insert(sub_block_id.clone(), sub_block);
                        Input::block(&sub_block_id)
                    }
                }
            }
            Expression::ReporterCall(call) => {
                let reporter_id = self.generate_id();
                let opcode = self.get_reporter_opcode(&call.category, &call.block_name);
                let mut reporter_block = Block::new(&opcode);
                reporter_block.parent = Some(parent_id.to_string());

                self.add_reporter_inputs(blocks, &mut reporter_block, call, &reporter_id);

                blocks.insert(reporter_id.clone(), reporter_block);
                Input::block(&reporter_id)
            }

            // List expressions
            Expression::ListLiteral(_) | Expression::MatrixLiteral(_) => {
                // List/matrix literals are handled during variable initialization
                // If used in an expression context, return empty string
                Input::literal(input_types::STRING, serde_json::json!(""))
            }

            Expression::IndexAccess { list, index } => {
                // data_itemoflist: get item at index from list
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_itemoflist");
                block.parent = Some(parent_id.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                // Convert 0-based index to 1-based for Scratch
                let index_plus_one = Expression::BinaryOp {
                    left: index.clone(),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                let index_input = self.generate_input(blocks, &index_plus_one, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                blocks.insert(block_id.clone(), block);
                Input::block(&block_id)
            }

            Expression::MatrixAccess { matrix, x, y } => {
                // For matrices stored as flattened lists, calculate: x + y * width + 1
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(matrix);
                let mut block = Block::new("data_itemoflist");
                block.parent = Some(parent_id.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(matrix, Some(&list_id)));

                // Calculate linear index: x + y * width + 1
                let width = self.matrix_widths.get(matrix).copied().unwrap_or(1) as i64;
                let index_expr = Expression::BinaryOp {
                    left: Box::new(Expression::BinaryOp {
                        left: x.clone(),
                        op: BinaryOperator::Add,
                        right: Box::new(Expression::BinaryOp {
                            left: y.clone(),
                            op: BinaryOperator::Mul,
                            right: Box::new(Expression::IntLiteral(width)),
                        }),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::IntLiteral(1)),
                };
                let index_input = self.generate_input(blocks, &index_expr, &block_id);
                block.inputs.insert("INDEX".to_string(), index_input);

                blocks.insert(block_id.clone(), block);
                Input::block(&block_id)
            }

            Expression::ListLength { list } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_lengthoflist");
                block.parent = Some(parent_id.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                blocks.insert(block_id.clone(), block);
                Input::block(&block_id)
            }

            Expression::ListContains { list, item } => {
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_listcontainsitem");
                block.parent = Some(parent_id.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                let item_input = self.generate_input(blocks, item, &block_id);
                block.inputs.insert("ITEM".to_string(), item_input);

                blocks.insert(block_id.clone(), block);
                Input::block(&block_id)
            }

            Expression::ListIndexOf { list, item } => {
                // data_itemnumoflist returns 1-based index, we convert to 0-based
                let block_id = self.generate_id();
                let list_id = self.get_or_create_list(list);
                let mut block = Block::new("data_itemnumoflist");
                block.parent = Some(parent_id.to_string());
                block
                    .fields
                    .insert("LIST".to_string(), Field::new(list, Some(&list_id)));

                let item_input = self.generate_input(blocks, item, &block_id);
                block.inputs.insert("ITEM".to_string(), item_input);

                blocks.insert(block_id.clone(), block);

                // Subtract 1 to convert from 1-based to 0-based index
                let sub_block_id = self.generate_id();
                let mut sub_block = Block::new("operator_subtract");
                sub_block.parent = Some(parent_id.to_string());
                sub_block
                    .inputs
                    .insert("NUM1".to_string(), Input::block(&block_id));
                sub_block.inputs.insert(
                    "NUM2".to_string(),
                    Input::literal(input_types::NUMBER, serde_json::json!("1")),
                );

                blocks.insert(sub_block_id.clone(), sub_block);
                Input::block(&sub_block_id)
            }
        }
    }

    fn generate_condition_input(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        expr: &Expression,
        parent_id: &str,
    ) -> Input {
        // For conditions, we need to handle boolean reporters and comparisons
        match expr {
            Expression::ReporterCall(call) => {
                let reporter_id = self.generate_id();
                let opcode = self.get_reporter_opcode(&call.category, &call.block_name);
                let mut reporter_block = Block::new(&opcode);
                reporter_block.parent = Some(parent_id.to_string());

                self.add_reporter_inputs(blocks, &mut reporter_block, call, &reporter_id);

                blocks.insert(reporter_id.clone(), reporter_block);
                Input::block(&reporter_id)
            }
            _ => self.generate_input(blocks, expr, parent_id),
        }
    }

    fn get_reporter_opcode(&self, category: &str, block_name: &str) -> String {
        (match (category, block_name) {
            // Motion reporters
            ("motion", "XPosition") => "motion_xposition",
            ("motion", "YPosition") => "motion_yposition",
            ("motion", "Direction") => "motion_direction",

            // Looks reporters
            ("looks", "CostumeNumber") => "looks_costumenumbername",
            ("looks", "BackdropNumber") => "looks_backdropnumbername",
            ("looks", "Size") => "looks_size",

            // Sound reporters
            ("sound", "Volume") => "sound_volume",

            // Sensing reporters
            ("sensing", "TouchingSprite") => "sensing_touchingobject",
            ("sensing", "TouchingColor") => "sensing_touchingcolor",
            ("sensing", "TouchingEdge") => "sensing_touchingobject",
            ("sensing", "DistanceTo") => "sensing_distanceto",
            ("sensing", "Answer") => "sensing_answer",
            ("sensing", "KeyPressed") => "sensing_keypressed",
            ("sensing", "MouseDown") => "sensing_mousedown",
            ("sensing", "MouseX") => "sensing_mousex",
            ("sensing", "MouseY") => "sensing_mousey",
            ("sensing", "Timer") => "sensing_timer",
            ("sensing", "Loudness") => "sensing_loudness",

            // Operators
            ("operators", "Random") => "operator_random",
            ("operators", "Join") => "operator_join",
            ("operators", "LetterOf") => "operator_letter_of",
            ("operators", "Length") => "operator_length",
            ("operators", "Contains") => "operator_contains",
            ("operators", "Mod") => "operator_mod",
            ("operators", "Round") => "operator_round",
            ("operators", "MathOp") => "operator_mathop",

            _ => {
                return format!("{}_{}", category, block_name.to_lowercase());
            }
        })
        .to_string()
    }

    fn add_reporter_inputs(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        block: &mut Block,
        call: &BlockCall,
        block_id: &str,
    ) {
        match (call.category.as_str(), call.block_name.as_str()) {
            ("sensing", "KeyPressed") => {
                if let Some(Expression::StringLiteral(key)) = call.args.first() {
                    let menu_id = self.generate_id();
                    let mut menu_block = Block::new("sensing_keyoptions");
                    menu_block.parent = Some(block_id.to_string());
                    menu_block.shadow = true;
                    menu_block
                        .fields
                        .insert("KEY_OPTION".to_string(), Field::new(key, None));
                    blocks.insert(menu_id.clone(), menu_block);
                    block
                        .inputs
                        .insert("KEY_OPTION".to_string(), Input::block(&menu_id));
                }
            }
            ("sensing", "TouchingSprite") => {
                if let Some(Expression::StringLiteral(sprite)) = call.args.first() {
                    let menu_id = self.generate_id();
                    let mut menu_block = Block::new("sensing_touchingobjectmenu");
                    menu_block.parent = Some(block_id.to_string());
                    menu_block.shadow = true;
                    menu_block
                        .fields
                        .insert("TOUCHINGOBJECTMENU".to_string(), Field::new(sprite, None));
                    blocks.insert(menu_id.clone(), menu_block);
                    block
                        .inputs
                        .insert("TOUCHINGOBJECTMENU".to_string(), Input::block(&menu_id));
                }
            }
            ("sensing", "TouchingEdge") => {
                let menu_id = self.generate_id();
                let mut menu_block = Block::new("sensing_touchingobjectmenu");
                menu_block.parent = Some(block_id.to_string());
                menu_block.shadow = true;
                menu_block
                    .fields
                    .insert("TOUCHINGOBJECTMENU".to_string(), Field::new("_edge_", None));
                blocks.insert(menu_id.clone(), menu_block);
                block
                    .inputs
                    .insert("TOUCHINGOBJECTMENU".to_string(), Input::block(&menu_id));
            }
            ("operators", "Random") => {
                if let Some(from) = call.args.first() {
                    let input = self.generate_input(blocks, from, block_id);
                    block.inputs.insert("FROM".to_string(), input);
                }
                if let Some(to) = call.args.get(1) {
                    let input = self.generate_input(blocks, to, block_id);
                    block.inputs.insert("TO".to_string(), input);
                }
            }
            ("operators", "Join") => {
                if let Some(s1) = call.args.first() {
                    let input = self.generate_input(blocks, s1, block_id);
                    block.inputs.insert("STRING1".to_string(), input);
                }
                if let Some(s2) = call.args.get(1) {
                    let input = self.generate_input(blocks, s2, block_id);
                    block.inputs.insert("STRING2".to_string(), input);
                }
            }
            ("operators", "Length") => {
                if let Some(s) = call.args.first() {
                    let input = self.generate_input(blocks, s, block_id);
                    block.inputs.insert("STRING".to_string(), input);
                }
            }
            ("operators", "Contains") => {
                // First argument is the string to search in
                if let Some(s1) = call.args.first() {
                    let input = self.generate_input(blocks, s1, block_id);
                    block.inputs.insert("STRING1".to_string(), input);
                }
                // Second argument is the string to search for
                if let Some(s2) = call.args.get(1) {
                    let input = self.generate_input(blocks, s2, block_id);
                    block.inputs.insert("STRING2".to_string(), input);
                }
            }
            ("operators", "Round") => {
                if let Some(num) = call.args.first() {
                    let input = self.generate_input(blocks, num, block_id);
                    block.inputs.insert("NUM".to_string(), input);
                }
            }
            ("operators", "MathOp") => {
                // First argument is the operation name (string)
                if let Some(Expression::StringLiteral(op_name)) = call.args.first() {
                    block
                        .fields
                        .insert("OPERATOR".to_string(), Field::new(op_name, None));
                }
                // Second argument is the number to operate on
                if let Some(num) = call.args.get(1) {
                    let input = self.generate_input(blocks, num, block_id);
                    block.inputs.insert("NUM".to_string(), input);
                }
            }
            _ => {}
        }
    }

    fn unwrap_unit_value<'a>(&self, expr: &'a Expression) -> &'a Expression {
        match expr {
            Expression::UnitValue { value, .. } => value,
            _ => expr,
        }
    }

    fn generate_power_operation(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        base: &Expression,
        exponent: &Expression,
        parent_id: &str,
    ) -> Input {
        // Check if exponent is a constant integer
        if let Expression::IntLiteral(exp) = exponent {
            if *exp >= 0 && *exp <= 100 {
                // For small positive integer exponents, generate repeated multiplication
                return self.generate_repeated_multiplication(
                    blocks,
                    base,
                    *exp as usize,
                    parent_id,
                );
            }
        }

        // For variable or large exponents, generate a loop with temporary variable
        self.generate_power_loop(blocks, base, exponent, parent_id)
    }

    fn generate_repeated_multiplication(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        base: &Expression,
        exponent: usize,
        parent_id: &str,
    ) -> Input {
        if exponent == 0 {
            // Any number to the power of 0 is 1
            return Input::literal(input_types::NUMBER, serde_json::json!("1"));
        }
        if exponent == 1 {
            // Any number to the power of 1 is itself
            return self.generate_input(blocks, base, parent_id);
        }

        // Build: base * base * base * ... (exponent times)
        // Start with base
        let base_input = self.generate_input(blocks, base, parent_id);
        let mut result = base_input.clone();

        // Multiply by base (exponent - 1) more times
        for _ in 1..exponent {
            let mul_block_id = self.generate_id();
            let mut mul_block = Block::new("operator_multiply");
            mul_block.parent = Some(parent_id.to_string());

            // Left side is the accumulated result
            mul_block.inputs.insert("NUM1".to_string(), result);
            // Right side is always the base
            mul_block
                .inputs
                .insert("NUM2".to_string(), base_input.clone());

            blocks.insert(mul_block_id.clone(), mul_block);
            result = Input::block(&mul_block_id);
        }

        result
    }

    fn generate_power_loop(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        base: &Expression,
        exponent: &Expression,
        parent_id: &str,
    ) -> Input {
        // For variable exponents, we need to create a loop structure.
        // Since we're in expression context and Scratch doesn't support loops in expressions,
        // we'll create a temporary variable and generate blocks that calculate the power.
        // However, these blocks need to be executed before the expression is evaluated.
        //
        // For now, we'll try to extract constant values from nested expressions,
        // and for truly variable exponents, we'll use a reasonable fallback.

        // Try to extract a constant integer value from the exponent expression
        let exp_value = self.extract_constant_int(exponent);
        if let Some(exp) = exp_value {
            if (0..=100).contains(&exp) {
                return self.generate_repeated_multiplication(
                    blocks,
                    base,
                    exp as usize,
                    parent_id,
                );
            }
        }

        // For variable or large exponents, use the mathematical identity:
        // base^exponent = e^(ln(base) * exponent)
        // This works for any positive base and any exponent value
        self.generate_power_via_exp_ln(blocks, base, exponent, parent_id)
    }

    fn generate_power_via_exp_ln(
        &mut self,
        blocks: &mut HashMap<String, Block>,
        base: &Expression,
        exponent: &Expression,
        parent_id: &str,
    ) -> Input {
        // Generate: e^(ln(base) * exponent)
        // This is the standard mathematical approach for computing arbitrary powers

        // Create ln(base) block
        let ln_block_id = self.generate_id();
        let mut ln_block = Block::new("operator_mathop");
        ln_block.parent = Some(parent_id.to_string());
        ln_block
            .fields
            .insert("OPERATOR".to_string(), Field::new("ln", None));
        let base_input = self.generate_input(blocks, base, &ln_block_id);
        ln_block.inputs.insert("NUM".to_string(), base_input);
        blocks.insert(ln_block_id.clone(), ln_block);

        // Create multiply block: ln(base) * exponent
        let mul_block_id = self.generate_id();
        let mut mul_block = Block::new("operator_multiply");
        mul_block.parent = Some(parent_id.to_string());
        mul_block
            .inputs
            .insert("NUM1".to_string(), Input::block(&ln_block_id));
        // Update ln_block parent to point to mul_block
        if let Some(ln_b) = blocks.get_mut(&ln_block_id) {
            ln_b.parent = Some(mul_block_id.clone());
        }
        let exp_input = self.generate_input(blocks, exponent, &mul_block_id);
        mul_block.inputs.insert("NUM2".to_string(), exp_input);
        blocks.insert(mul_block_id.clone(), mul_block);

        // Create e^ block: e^(ln(base) * exponent)
        let exp_block_id = self.generate_id();
        let mut exp_block = Block::new("operator_mathop");
        exp_block.parent = Some(parent_id.to_string());
        exp_block
            .fields
            .insert("OPERATOR".to_string(), Field::new("e ^", None));
        exp_block
            .inputs
            .insert("NUM".to_string(), Input::block(&mul_block_id));
        // Update mul_block parent to point to exp_block
        if let Some(mul_b) = blocks.get_mut(&mul_block_id) {
            mul_b.parent = Some(exp_block_id.clone());
        }
        blocks.insert(exp_block_id.clone(), exp_block);

        Input::block(&exp_block_id)
    }

    fn extract_constant_int(&self, expr: &Expression) -> Option<i64> {
        match expr {
            Expression::IntLiteral(n) => Some(*n),
            Expression::FloatLiteral(n) => Some(*n as i64),
            Expression::UnaryOp { op, operand } => {
                if let UnaryOperator::Neg = op {
                    self.extract_constant_int(operand).map(|n| -n)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub fn generate(config: &Config, ast: &Program) -> ScratchProject {
    let mut generator = CodeGenerator::new();
    generator.generate(config, ast)
}

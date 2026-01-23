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
    /// Map of broadcast names to their IDs
    broadcasts: HashMap<String, String>,
    /// Current Y position for placing top-level blocks
    current_y: f64,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            block_counter: 0,
            variables: HashMap::new(),
            broadcasts: HashMap::new(),
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
            let sprite = self.generate_sprite(sprite_node, i + 1);
            project.targets.push(sprite);
        }

        // Add broadcasts to stage
        if let Some(stage) = project.targets.first_mut() {
            for (name, id) in &self.broadcasts {
                stage.broadcasts.insert(id.clone(), name.clone());
            }
        }

        project
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
            let var_id = self.get_or_create_variable(&global.name);
            let initial_value = self.expr_to_json_value(&global.initial_value);
            target
                .variables
                .insert(var_id, Variable::new(&global.name, initial_value));
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

    fn generate_stage(&mut self, stage_node: &StageNode, globals: &[GlobalVar]) -> Target {
        let mut stage = Target::new_stage();
        self.add_global_variables(&mut stage, globals);
        self.current_y = 0.0;

        if let Some(code) = &stage_node.code {
            self.generate_code_block(&mut stage.blocks, code);
        }

        stage
    }

    fn generate_sprite(&mut self, sprite_node: &SpriteNode, layer_order: usize) -> Target {
        let mut sprite = Target::new_sprite(&sprite_node.name, layer_order);
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
            proccode: Some(proccode),
            argumentids: Some(serde_json::to_string(&argument_ids).unwrap()),
            argumentnames: Some(serde_json::to_string(&argument_names).unwrap()),
            argumentdefaults: Some(serde_json::to_string(&argument_defaults).unwrap()),
            warp: Some("false".to_string()),
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

            Statement::ChangeVariable { name, value } => {
                let block_id = self.generate_id();
                let var_id = self.get_or_create_variable(name);
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
            ("motion", "ChangeX") | ("motion", "SetX") => {
                if let Some(x) = call.args.first() {
                    let input = self.generate_input(blocks, x, block_id);
                    block.inputs.insert("X".to_string(), input);
                }
            }
            ("motion", "ChangeY") | ("motion", "SetY") => {
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
                            Input::literal(input_types::NUMBER, serde_json::json!("0")),
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
            _ => {}
        }
    }

    fn unwrap_unit_value<'a>(&self, expr: &'a Expression) -> &'a Expression {
        match expr {
            Expression::UnitValue { value, .. } => value,
            _ => expr,
        }
    }
}

pub fn generate(config: &Config, ast: &Program) -> ScratchProject {
    let mut generator = CodeGenerator::new();
    generator.generate(config, ast)
}

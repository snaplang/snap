use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn current_kind(&self) -> &TokenKind {
        &self.tokens[self.pos].kind
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: TokenKind) -> Result<&Token, String> {
        if std::mem::discriminant(self.current_kind()) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at {}:{}",
                expected,
                self.current_kind(),
                self.current().line,
                self.current().column
            ))
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.current_kind()) == std::mem::discriminant(kind)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut imports = Vec::new();
        let mut extensions = Vec::new();
        let mut globals = Vec::new();
        let mut stage = None;
        let mut sprites = Vec::new();

        while !self.check(&TokenKind::Eof) {
            match self.current_kind() {
                TokenKind::Use => {
                    // Check if it's "use pen" or a file import
                    let next_pos = self.pos + 1;
                    if next_pos < self.tokens.len() {
                        if let TokenKind::Identifier(ref ident) = &self.tokens[next_pos].kind {
                            if ident == "pen" {
                                self.advance(); // consume "use"
                                self.advance(); // consume "pen"
                                self.expect(TokenKind::Semicolon)?;
                                extensions.push("pen".to_string());
                                continue;
                            }
                        }
                    }
                    imports.push(self.parse_import()?);
                }
                TokenKind::Let => {
                    globals.push(self.parse_global_var()?);
                }
                TokenKind::New => {
                    self.advance(); // consume 'new'
                    match self.current_kind() {
                        TokenKind::Stage => {
                            if stage.is_some() {
                                return Err("Multiple Stage definitions not allowed".to_string());
                            }
                            stage = Some(self.parse_stage()?);
                        }
                        TokenKind::Sprite => {
                            sprites.push(self.parse_sprite()?);
                        }
                        _ => {
                            return Err(format!(
                                "Expected 'Stage' or 'Sprite' after 'new' at {}:{}",
                                self.current().line,
                                self.current().column
                            ));
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Unexpected token {:?} at {}:{}",
                        self.current_kind(),
                        self.current().line,
                        self.current().column
                    ));
                }
            }
        }

        Ok(Program {
            imports,
            extensions,
            globals,
            stage,
            sprites,
        })
    }

    fn parse_import(&mut self) -> Result<Import, String> {
        self.expect(TokenKind::Use)?;
        let path = self.parse_string_literal()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Import { path })
    }

    fn parse_global_var(&mut self) -> Result<GlobalVar, String> {
        self.expect(TokenKind::Let)?;
        let name = self.parse_identifier()?;
        self.expect(TokenKind::Colon)?;
        let var_type = self.parse_var_type()?;
        self.expect(TokenKind::Equals)?;
        let initial_value = self.parse_expression()?;

        // Parse optional monitor configuration
        let (monitor_x, monitor_y, monitor_visible) =
            if let TokenKind::Identifier(ref ident) = self.current_kind() {
                if ident == "monitor" {
                    self.advance(); // consume "monitor"
                    self.parse_monitor_config()?
                } else {
                    (None, None, None)
                }
            } else {
                (None, None, None)
            };

        self.expect(TokenKind::Semicolon)?;

        Ok(GlobalVar {
            name,
            var_type,
            initial_value,
            monitor_x,
            monitor_y,
            monitor_visible,
        })
    }

    fn parse_monitor_config(&mut self) -> Result<(Option<f64>, Option<f64>, Option<bool>), String> {
        self.expect(TokenKind::LParen)?;

        let mut x = None;
        let mut y = None;
        let mut visible = None;

        while !self.check(&TokenKind::RParen) {
            let key = self.parse_identifier()?;
            self.expect(TokenKind::Colon)?;

            match key.as_str() {
                "x" => {
                    let value = self.parse_number()?;
                    x = Some(value);
                }
                "y" => {
                    let value = self.parse_number()?;
                    y = Some(value);
                }
                "visible" => {
                    let value = match self.current_kind() {
                        TokenKind::True => {
                            self.advance();
                            true
                        }
                        TokenKind::False => {
                            self.advance();
                            false
                        }
                        _ => {
                            return Err(format!(
                                "Expected true or false for visible, found {:?} at {}:{}",
                                self.current_kind(),
                                self.current().line,
                                self.current().column
                            ));
                        }
                    };
                    visible = Some(value);
                }
                _ => {
                    return Err(format!(
                        "Unknown monitor config key: {} at {}:{}",
                        key,
                        self.current().line,
                        self.current().column
                    ));
                }
            }

            if !self.check(&TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        self.expect(TokenKind::RParen)?;

        Ok((x, y, visible))
    }

    fn parse_var_type(&mut self) -> Result<VarType, String> {
        match self.current_kind() {
            TokenKind::Int => {
                self.advance();
                Ok(VarType::Int)
            }
            TokenKind::Float => {
                self.advance();
                Ok(VarType::Float)
            }
            TokenKind::Bool => {
                self.advance();
                Ok(VarType::Bool)
            }
            TokenKind::String => {
                self.advance();
                Ok(VarType::String)
            }
            _ => Err(format!(
                "Expected type, found {:?} at {}:{}",
                self.current_kind(),
                self.current().line,
                self.current().column
            )),
        }
    }

    fn parse_stage(&mut self) -> Result<StageNode, String> {
        self.expect(TokenKind::Stage)?;
        self.expect(TokenKind::LBrace)?;

        let mut backdrops = Vec::new();
        let mut code = None;

        while !self.check(&TokenKind::RBrace) {
            if let TokenKind::Identifier(ref ident) = self.current_kind() {
                if ident == "backdrops" {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    backdrops = self.parse_string_array()?;
                    // Optional comma
                    self.match_token(&TokenKind::Comma);
                } else {
                    return Err(format!(
                        "Unexpected identifier in Stage: {} at {}:{}",
                        ident,
                        self.current().line,
                        self.current().column
                    ));
                }
            } else if self.check(&TokenKind::Implements) {
                code = Some(self.parse_implements_code()?);
            } else {
                return Err(format!(
                    "Unexpected token in Stage: {:?} at {}:{}",
                    self.current_kind(),
                    self.current().line,
                    self.current().column
                ));
            }
        }

        self.expect(TokenKind::RBrace)?;

        Ok(StageNode { backdrops, code })
    }

    fn parse_sprite(&mut self) -> Result<SpriteNode, String> {
        self.expect(TokenKind::Sprite)?;
        self.expect(TokenKind::LParen)?;
        let name = self.parse_string_literal()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut costumes = Vec::new();
        let mut position = None;
        let mut size = None;
        let mut code = None;

        while !self.check(&TokenKind::RBrace) {
            if let TokenKind::Identifier(ref ident) = self.current_kind() {
                if ident == "costumes" {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    costumes = self.parse_string_array()?;
                    self.match_token(&TokenKind::Comma);
                } else if ident == "position" {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    position = Some(self.parse_position()?);
                    self.match_token(&TokenKind::Comma);
                } else if ident == "size" {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    size = Some(self.parse_number()?);
                    self.match_token(&TokenKind::Comma);
                } else {
                    return Err(format!(
                        "Unexpected identifier in Sprite: {} at {}:{}",
                        ident,
                        self.current().line,
                        self.current().column
                    ));
                }
            } else if self.check(&TokenKind::Implements) {
                code = Some(self.parse_implements_code()?);
            } else {
                return Err(format!(
                    "Unexpected token in Sprite: {:?} at {}:{}",
                    self.current_kind(),
                    self.current().line,
                    self.current().column
                ));
            }
        }

        self.expect(TokenKind::RBrace)?;

        Ok(SpriteNode {
            name,
            costumes,
            position,
            size,
            code,
        })
    }

    fn parse_implements_code(&mut self) -> Result<CodeBlock, String> {
        self.expect(TokenKind::Implements)?;
        self.expect(TokenKind::Code)?;
        self.expect(TokenKind::LBrace)?;

        let mut event_handlers = Vec::new();
        let mut functions = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            if self.check(&TokenKind::On) {
                event_handlers.push(self.parse_event_handler()?);
            } else if self.check(&TokenKind::Fn) {
                functions.push(self.parse_function()?);
            } else {
                return Err(format!(
                    "Expected 'on' or 'fn' in Code block, found {:?} at {}:{}",
                    self.current_kind(),
                    self.current().line,
                    self.current().column
                ));
            }
        }

        self.expect(TokenKind::RBrace)?;

        Ok(CodeBlock {
            event_handlers,
            functions,
        })
    }

    fn parse_event_handler(&mut self) -> Result<EventHandler, String> {
        self.expect(TokenKind::On)?;
        let event = self.parse_event()?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_statements()?;
        self.expect(TokenKind::RBrace)?;

        Ok(EventHandler { event, body })
    }

    fn parse_event(&mut self) -> Result<Event, String> {
        let ident = self.parse_identifier()?;
        match ident.as_str() {
            "GreenFlag" => Ok(Event::GreenFlag),
            "Clicked" => Ok(Event::Clicked),
            "CloneStart" => Ok(Event::CloneStart),
            "KeyPressed" => {
                self.expect(TokenKind::LParen)?;
                let key = self.parse_string_literal()?;
                self.expect(TokenKind::RParen)?;
                Ok(Event::KeyPressed(key))
            }
            "Broadcast" => {
                self.expect(TokenKind::LParen)?;
                let msg = self.parse_string_literal()?;
                self.expect(TokenKind::RParen)?;
                Ok(Event::Broadcast(msg))
            }
            "BackdropSwitch" => {
                self.expect(TokenKind::LParen)?;
                let backdrop = self.parse_string_literal()?;
                self.expect(TokenKind::RParen)?;
                Ok(Event::BackdropSwitch(backdrop))
            }
            _ => Err(format!("Unknown event type: {}", ident)),
        }
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(TokenKind::Fn)?;

        // Check for optional 'warp' keyword after 'fn'
        let warp = if self.check(&TokenKind::Warp) {
            self.advance();
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;
        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) {
            let param_name = self.parse_identifier()?;
            self.expect(TokenKind::Colon)?;
            let param_type = self.parse_var_type()?;
            params.push(Parameter {
                name: param_name,
                param_type,
            });
            if !self.check(&TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_statements()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Function {
            name,
            params,
            body,
            warp,
        })
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        // Check for control flow keywords
        if self.check(&TokenKind::Set) {
            return self.parse_set_variable();
        }
        if self.check(&TokenKind::Change) {
            return self.parse_change_variable();
        }
        if self.check(&TokenKind::If) {
            return self.parse_if_statement();
        }

        // Check for identifier (could be a block call, function call, or control block)
        if let TokenKind::Identifier(_ident) = self.current_kind().clone() {
            // Check if it's a namespaced block call (e.g., motion::Move)
            let next_pos = self.pos + 1;
            if next_pos < self.tokens.len() {
                if let TokenKind::ColonColon = &self.tokens[next_pos].kind {
                    return self.parse_block_call_statement();
                }
            }

            // Otherwise it might be a function call or special construct
            return self.parse_function_call_statement();
        }

        Err(format!(
            "Unexpected token in statement: {:?} at {}:{}",
            self.current_kind(),
            self.current().line,
            self.current().column
        ))
    }

    fn parse_set_variable(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Set)?;
        let name = self.parse_identifier()?;
        self.expect(TokenKind::Equals)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::SetVariable { name, value })
    }

    fn parse_change_variable(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Change)?;
        let name = self.parse_identifier()?;

        // Expect "by" as an identifier
        let by_keyword = self.parse_identifier()?;
        if by_keyword != "by" {
            return Err("Expected 'by' after variable name in change statement".to_string());
        }

        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::ChangeVariable { name, value })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::If)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let then_body = self.parse_statements()?;
        self.expect(TokenKind::RBrace)?;

        let else_body = if self.check(&TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            let body = self.parse_statements()?;
            self.expect(TokenKind::RBrace)?;
            Some(body)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_block_call_statement(&mut self) -> Result<Statement, String> {
        let block_call = self.parse_block_call()?;

        // Check for special control blocks that have bodies
        if block_call.category == "control" {
            match block_call.block_name.as_str() {
                "Forever" => {
                    self.expect(TokenKind::LBrace)?;
                    let body = self.parse_statements()?;
                    self.expect(TokenKind::RBrace)?;
                    return Ok(Statement::Forever { body });
                }
                "Repeat" => {
                    if block_call.args.len() != 1 {
                        return Err("Repeat requires exactly one argument".to_string());
                    }
                    self.expect(TokenKind::LBrace)?;
                    let body = self.parse_statements()?;
                    self.expect(TokenKind::RBrace)?;
                    return Ok(Statement::Repeat {
                        times: block_call.args[0].clone(),
                        body,
                    });
                }
                "RepeatUntil" => {
                    if block_call.args.len() != 1 {
                        return Err("RepeatUntil requires exactly one argument".to_string());
                    }
                    self.expect(TokenKind::LBrace)?;
                    let body = self.parse_statements()?;
                    self.expect(TokenKind::RBrace)?;
                    return Ok(Statement::RepeatUntil {
                        condition: block_call.args[0].clone(),
                        body,
                    });
                }
                "Wait" => {
                    self.expect(TokenKind::Semicolon)?;
                    if block_call.args.len() != 1 {
                        return Err("Wait requires exactly one argument".to_string());
                    }
                    return Ok(Statement::Wait {
                        duration: block_call.args[0].clone(),
                    });
                }
                "Stop" => {
                    self.expect(TokenKind::Semicolon)?;
                    if let Some(Expression::StringLiteral(mode)) = block_call.args.first() {
                        return Ok(Statement::Stop { mode: mode.clone() });
                    } else {
                        return Err("Stop requires a string argument".to_string());
                    }
                }
                "CreateClone" => {
                    self.expect(TokenKind::Semicolon)?;
                    if let Some(Expression::StringLiteral(target)) = block_call.args.first() {
                        return Ok(Statement::CreateClone {
                            target: target.clone(),
                        });
                    } else {
                        return Err("CreateClone requires a string argument".to_string());
                    }
                }
                "DeleteClone" => {
                    self.expect(TokenKind::Semicolon)?;
                    return Ok(Statement::DeleteClone);
                }
                _ => {}
            }
        }

        // Regular block call
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::BlockCall(block_call))
    }

    fn parse_function_call_statement(&mut self) -> Result<Statement, String> {
        let name = self.parse_identifier()?;
        self.expect(TokenKind::LParen)?;

        let mut args = Vec::new();
        while !self.check(&TokenKind::RParen) {
            args.push(self.parse_expression()?);
            if !self.check(&TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::FunctionCall { name, args })
    }

    fn parse_block_call(&mut self) -> Result<BlockCall, String> {
        let category = self.parse_identifier()?;
        self.expect(TokenKind::ColonColon)?;
        let block_name = self.parse_identifier()?;

        // Check if there are arguments
        let args = if self.check(&TokenKind::LParen) {
            self.expect(TokenKind::LParen)?;
            let mut args = Vec::new();
            while !self.check(&TokenKind::RParen) {
                args.push(self.parse_expression()?);
                if !self.check(&TokenKind::RParen) {
                    self.expect(TokenKind::Comma)?;
                }
            }
            self.expect(TokenKind::RParen)?;
            args
        } else {
            Vec::new()
        };

        Ok(BlockCall {
            category,
            block_name,
            args,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and_expr()?;

        while self.check(&TokenKind::OrOr) {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality_expr()?;

        while self.check(&TokenKind::AndAnd) {
            self.advance();
            let right = self.parse_equality_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison_expr()?;

        loop {
            let op = match self.current_kind() {
                TokenKind::EqEq => BinaryOperator::Eq,
                TokenKind::BangEq => BinaryOperator::NotEq,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_comparison_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_additive_expr()?;

        loop {
            let op = match self.current_kind() {
                TokenKind::Lt => BinaryOperator::Lt,
                TokenKind::Gt => BinaryOperator::Gt,
                TokenKind::LtEq => BinaryOperator::LtEq,
                TokenKind::GtEq => BinaryOperator::GtEq,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_additive_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative_expr()?;

        loop {
            let op = match self.current_kind() {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Sub,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_multiplicative_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_power_expr()?;

        loop {
            let op = match self.current_kind() {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                TokenKind::Percent => BinaryOperator::Mod,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_power_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power_expr(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary_expr()?;

        // Power is right-associative: a^b^c = a^(b^c)
        while self.check(&TokenKind::Caret) {
            self.advance();
            let right = self.parse_power_expr()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Power,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Expression, String> {
        match self.current_kind() {
            TokenKind::Bang => {
                self.advance();
                let operand = self.parse_unary_expr()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_unary_expr()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Neg,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary_expr(),
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Expression, String> {
        match self.current_kind().clone() {
            TokenKind::IntLiteral(n) => {
                self.advance();
                Ok(Expression::IntLiteral(n))
            }
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Ok(Expression::FloatLiteral(n))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(Expression::StringLiteral(s))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expression::BoolLiteral(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expression::BoolLiteral(false))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::Identifier(ident) => {
                // Check if it's a namespaced call (reporter or unit)
                let next_pos = self.pos + 1;
                if next_pos < self.tokens.len() {
                    if let TokenKind::ColonColon = &self.tokens[next_pos].kind {
                        // Could be a reporter call or unit value
                        let category = ident.clone();
                        self.advance();
                        self.expect(TokenKind::ColonColon)?;
                        let name = self.parse_identifier()?;

                        // Check for units
                        if category == "units" {
                            self.expect(TokenKind::LParen)?;

                            // Special handling for Rgb and Rgba which take multiple arguments
                            if name == "Rgb" {
                                let r = self.parse_expression()?;
                                self.expect(TokenKind::Comma)?;
                                let g = self.parse_expression()?;
                                self.expect(TokenKind::Comma)?;
                                let b = self.parse_expression()?;
                                self.expect(TokenKind::RParen)?;

                                // Calculate: r + g*256 + b*65536
                                let g_times_256 = Expression::BinaryOp {
                                    left: Box::new(g),
                                    op: BinaryOperator::Mul,
                                    right: Box::new(Expression::IntLiteral(256)),
                                };
                                let b_times_65536 = Expression::BinaryOp {
                                    left: Box::new(b),
                                    op: BinaryOperator::Mul,
                                    right: Box::new(Expression::IntLiteral(65536)),
                                };
                                let gb_sum = Expression::BinaryOp {
                                    left: Box::new(g_times_256),
                                    op: BinaryOperator::Add,
                                    right: Box::new(b_times_65536),
                                };
                                let rgb_sum = Expression::BinaryOp {
                                    left: Box::new(r),
                                    op: BinaryOperator::Add,
                                    right: Box::new(gb_sum),
                                };

                                return Ok(Expression::UnitValue {
                                    unit: name,
                                    value: Box::new(rgb_sum),
                                });
                            } else if name == "Rgba" {
                                let r = self.parse_expression()?;
                                self.expect(TokenKind::Comma)?;
                                let g = self.parse_expression()?;
                                self.expect(TokenKind::Comma)?;
                                let b = self.parse_expression()?;
                                self.expect(TokenKind::Comma)?;
                                let a = self.parse_expression()?;
                                self.expect(TokenKind::RParen)?;

                                // Calculate: r + g*256 + b*65536 + a*16777216
                                let g_times_256 = Expression::BinaryOp {
                                    left: Box::new(g),
                                    op: BinaryOperator::Mul,
                                    right: Box::new(Expression::IntLiteral(256)),
                                };
                                let b_times_65536 = Expression::BinaryOp {
                                    left: Box::new(b),
                                    op: BinaryOperator::Mul,
                                    right: Box::new(Expression::IntLiteral(65536)),
                                };
                                let a_times_16777216 = Expression::BinaryOp {
                                    left: Box::new(a),
                                    op: BinaryOperator::Mul,
                                    right: Box::new(Expression::IntLiteral(16777216)),
                                };
                                let gb_sum = Expression::BinaryOp {
                                    left: Box::new(g_times_256),
                                    op: BinaryOperator::Add,
                                    right: Box::new(b_times_65536),
                                };
                                let rgb_sum = Expression::BinaryOp {
                                    left: Box::new(r),
                                    op: BinaryOperator::Add,
                                    right: Box::new(gb_sum),
                                };
                                let rgba_sum = Expression::BinaryOp {
                                    left: Box::new(rgb_sum),
                                    op: BinaryOperator::Add,
                                    right: Box::new(a_times_16777216),
                                };

                                return Ok(Expression::UnitValue {
                                    unit: name,
                                    value: Box::new(rgba_sum),
                                });
                            } else {
                                // Regular unit with single argument
                                let value = self.parse_expression()?;
                                self.expect(TokenKind::RParen)?;
                                return Ok(Expression::UnitValue {
                                    unit: name,
                                    value: Box::new(value),
                                });
                            }
                        }

                        // Reporter call with optional args
                        let args = if self.check(&TokenKind::LParen) {
                            self.expect(TokenKind::LParen)?;
                            let mut args = Vec::new();
                            while !self.check(&TokenKind::RParen) {
                                args.push(self.parse_expression()?);
                                if !self.check(&TokenKind::RParen) {
                                    self.expect(TokenKind::Comma)?;
                                }
                            }
                            self.expect(TokenKind::RParen)?;
                            args
                        } else {
                            Vec::new()
                        };

                        return Ok(Expression::ReporterCall(BlockCall {
                            category,
                            block_name: name,
                            args,
                        }));
                    }
                }

                // Just a variable reference
                self.advance();
                Ok(Expression::Variable(ident))
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?} at {}:{}",
                self.current_kind(),
                self.current().line,
                self.current().column
            )),
        }
    }

    // Helper functions

    fn parse_identifier(&mut self) -> Result<String, String> {
        if let TokenKind::Identifier(name) = self.current_kind().clone() {
            self.advance();
            Ok(name)
        } else {
            Err(format!(
                "Expected identifier, found {:?} at {}:{}",
                self.current_kind(),
                self.current().line,
                self.current().column
            ))
        }
    }

    fn parse_string_literal(&mut self) -> Result<String, String> {
        if let TokenKind::StringLiteral(s) = self.current_kind().clone() {
            self.advance();
            Ok(s)
        } else {
            Err(format!(
                "Expected string literal, found {:?} at {}:{}",
                self.current_kind(),
                self.current().line,
                self.current().column
            ))
        }
    }

    fn parse_string_array(&mut self) -> Result<Vec<String>, String> {
        self.expect(TokenKind::LBracket)?;
        let mut items = Vec::new();

        while !self.check(&TokenKind::RBracket) {
            items.push(self.parse_string_literal()?);
            if !self.check(&TokenKind::RBracket) {
                self.expect(TokenKind::Comma)?;
            }
        }

        self.expect(TokenKind::RBracket)?;
        Ok(items)
    }

    fn parse_position(&mut self) -> Result<(f64, f64), String> {
        self.expect(TokenKind::LParen)?;
        let x = self.parse_number()?;
        self.expect(TokenKind::Comma)?;
        let y = self.parse_number()?;
        self.expect(TokenKind::RParen)?;
        Ok((x, y))
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let negative = if self.check(&TokenKind::Minus) {
            self.advance();
            true
        } else {
            false
        };

        let value = match self.current_kind().clone() {
            TokenKind::IntLiteral(n) => {
                self.advance();
                n as f64
            }
            TokenKind::FloatLiteral(n) => {
                self.advance();
                n
            }
            _ => {
                return Err(format!(
                    "Expected number, found {:?} at {}:{}",
                    self.current_kind(),
                    self.current().line,
                    self.current().column
                ));
            }
        };

        Ok(if negative { -value } else { value })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

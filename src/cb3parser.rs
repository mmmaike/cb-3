use crate::C1Lexer;
use crate::C1Token;
use crate::ParseResult;

pub struct C1Parser<'a> {
    lexer: C1Lexer<'a>,
}

impl C1Parser<'_> {
    pub fn parse(text: &str) -> ParseResult {
        let mut parser: C1Parser = C1Parser {
            lexer: C1Lexer::new(text),
        };

        parser.program()
    }

    pub fn check_and_eat_token(&mut self, token: C1Token) -> ParseResult {
        if self.current_matches(token) {
            self.eat();
            Result::Ok(())
        } else {
            self.error_message(token)
        }
    }

    fn current_token(&self) -> C1Token {
        self.lexer.current_token().unwrap()
    }

    fn eat(&mut self) {
        self.lexer.eat();
    }

    fn current_matches(&self, token: C1Token) -> bool {
        let current = self.lexer.current_token().unwrap();
        if current == token {
            return true;
        }
        false
    }

    pub fn next_matches(&self, token: C1Token) -> bool {
        let next = self.lexer.peek_token().unwrap();
        if next == token {
            return true;
        }
        false
    }

    fn program(&mut self) -> ParseResult {
        let mut res = Result::Ok(());
        let mut fail = false;

        while !fail && self.lexer.current_token().is_some() {
            res = self.function_definition(self.current_token());
            fail = res.is_err();
        }
        res
    }

    fn function_definition(&mut self, token: C1Token) -> ParseResult {
        //This only works if the last call (check and eat RightBrace) happens as a separate step from the others?
        let _ = self
            .type_kw(token)
            .and(self.check_and_eat_token(C1Token::Identifier))
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
            .and(self.check_and_eat_token(C1Token::LeftBrace))
            .and(self.statementlist(self.current_token()));
        self.check_and_eat_token(C1Token::RightBrace)
    }

    fn function_call(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::Identifier
            && self.lexer.peek_token().unwrap() == C1Token::LeftParenthesis
        {
            return self
                .check_and_eat_token(token)
                .and(self.check_and_eat_token(C1Token::LeftParenthesis))
                .and(self.check_and_eat_token(C1Token::RightParenthesis));
        }
        self.error_message(token)
    }

    fn statementlist(&mut self, token: C1Token) -> ParseResult {
        let mut res = Result::Ok(());
        let mut fail = false;
        while !fail {
            res = self.block(token);
            if res.is_err() {
                fail = true;
            }
        }
        res
    }

    fn block(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::LeftBrace {
            let _ = self
                .check_and_eat_token(C1Token::LeftBrace)
                .and(self.statementlist(self.current_token()));

            self.check_and_eat_token(C1Token::RightBrace)
        } else if token == C1Token::Identifier
            || token == C1Token::KwIf
            || token == C1Token::KwReturn
            || token == C1Token::KwPrintf
        {
            self.statement(self.current_token())
        } else {
            self.error_message(token)
        }
    }

    fn statement(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::KwIf {
            return self.if_statement();
        }

        match token {
            C1Token::KwReturn => self.return_statement(),
            C1Token::KwPrintf => self
                .printf()
                .and(self.check_and_eat_token(C1Token::Semicolon)),
            C1Token::Identifier => {
                if self.next_matches(C1Token::Assign) {
                    self.call_statassign()
                } else if self.next_matches(C1Token::LeftParenthesis) {
                    self.function_call(token)
                        .and(self.check_and_eat_token(C1Token::Semicolon))
                } else {
                    self.error_message(token)
                        .and(self.check_and_eat_token(C1Token::Semicolon))
                }
            }
            _ => self.error_message(token),
        }
    }

    fn call_statassign(&mut self) -> ParseResult {
        let res = self.stat_assignment();
        res.as_ref()?;
        self.check_and_eat_token(C1Token::Semicolon)
    }

    fn if_statement(&mut self) -> ParseResult {
        self.check_and_eat_token(C1Token::KwIf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.assignment(self.current_token()))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
            .and(self.block(self.current_token()))
    }

    fn return_statement(&mut self) -> ParseResult {
        let intermediate_result = self.check_and_eat_token(C1Token::KwReturn);

        if self.assignment(self.lexer.current_token().unwrap()).is_ok() {
            self.check_and_eat_token(C1Token::Semicolon)
        } else {
            intermediate_result
        }
    }

    fn printf(&mut self) -> ParseResult {
        self.check_and_eat_token(C1Token::KwPrintf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.assignment(self.current_token()))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
    }

    fn type_kw(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::KwBoolean
            || token == C1Token::KwFloat
            || token == C1Token::KwInt
            || token == C1Token::KwVoid
        {
            self.check_and_eat_token(token)
        } else {
            self.error_message(token)
        }
    }

    fn stat_assignment(&mut self) -> ParseResult {
        self.check_and_eat_token(C1Token::Identifier)
            .and(self.check_and_eat_token(C1Token::Assign))
            .and(self.assignment(self.current_token()))
    }

    fn assignment(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::Identifier && self.next_matches(C1Token::Assign) {
            self.check_and_eat_token(token)
                .and(self.check_and_eat_token(C1Token::Assign))
                .and(self.assignment(self.lexer.current_token().unwrap()))
        } else {
            self.expr()
        }
    }

    fn expr(&mut self) -> ParseResult {
        let result = self.simpexpr();

        if self.current_matches(C1Token::Equal)
            || self.current_matches(C1Token::NotEqual)
            || self.current_matches(C1Token::LessEqual)
            || self.current_matches(C1Token::GreaterEqual)
            || self.current_matches(C1Token::Less)
            || self.current_matches(C1Token::Greater)
        {
            self.check_and_eat_token(self.lexer.current_token().unwrap())
                .and(self.simpexpr())
        } else {
            result
        }
    }

    fn simpexpr(&mut self) -> ParseResult {
        if self.current_matches(C1Token::Minus) {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
        }
        let mut res = self.term(self.current_token());

        let mut second_term = self.current_matches(C1Token::Plus)
            || self.current_matches(C1Token::Minus)
            || self.current_matches(C1Token::Or);

        while second_term {
            let _ = self.check_and_eat_token(self.current_token());
            res = self.term(self.current_token());

            second_term = self.current_matches(C1Token::Plus)
                || self.current_matches(C1Token::Minus)
                || self.current_matches(C1Token::Or);
        }

        res
    }

    fn term(&mut self, token: C1Token) -> ParseResult {
        let intermediate_result = self.factor(token);

        if self.current_matches(C1Token::Asterisk)
            || self.current_matches(C1Token::Slash)
            || self.current_matches(C1Token::And)
        {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            return self.factor(self.lexer.current_token().unwrap());
        }

        intermediate_result
    }

    fn factor(&mut self, token: C1Token) -> ParseResult {
        if token == C1Token::ConstInt
            || token == C1Token::ConstFloat
            || token == C1Token::ConstBoolean
        {
            self.check_and_eat_token(token)
        } else if token == C1Token::Identifier {
            if self.next_matches(C1Token::LeftParenthesis) {
                self.function_call(token)
            } else {
                self.check_and_eat_token(token)
            }
        } else if token == C1Token::LeftParenthesis {
            self.check_and_eat_token(token)
                .and(self.assignment(self.current_token()))
                .and(self.check_and_eat_token(C1Token::RightParenthesis))
        } else {
            self.error_message(token)
        }
    }

    fn error_message(&mut self, token: C1Token) -> ParseResult {
        let msg = format!(
            "Token mismatch in line {:?} on token {:?}, expected {:?}, was {:?}",
            self.lexer.current_line_number().unwrap(),
            self.lexer.current_text().unwrap(),
            token,
            self.current_token()
        );
        Result::Err(msg)
    }
}

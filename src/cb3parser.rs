use std::fmt;

use crate::lexer;
use crate::C1Lexer;
use crate::C1Token;
use crate::ParseResult;

pub struct C1Parser<'a> {
    lexer: C1Lexer<'a>,
}

//Lexer offers:
/*  peek token
   current text
   peek text
   current line number
   peek line number

*/

impl C1Parser<'_> {
    pub fn parse(text: &str) -> ParseResult {
        //init lexer here, not in C1Parser struct/type - C1Parser does not get initiliazd, only ass. func.
        let mut parser: C1Parser = C1Parser {
            lexer: C1Lexer::new(text),
        };
        let token = parser.lexer.current_token().unwrap();
        parser.program(token)
    }

    /*
    Implementieren (und benutzen) Sie eine Funktion/Methode namens check_and_eat_token(token: C1Token),
     die überprüft, ob das ihr übergebene Token gleich dem aktuellen ist.
      Im Positivfall wird das aktuelle Token konsumiert,
      im Negativfall wird ein Fehler zurückgegeben.
    */
    pub fn check_and_eat_token(&mut self, token: C1Token) -> ParseResult {
        let current = self.lexer.current_token().unwrap();
        println!("=== BEGIN CHECK AND EAT OUTPUT =======");
        println!("current: {:?} ", current);
        println!("given: {:?} ", token);
        println!("current text: {:?}", self.lexer.current_text());

        println!(
            "next text: {:?}, type: {:?}",
            self.lexer.peek_text(),
            self.lexer.peek_token()
        );

        if self.current_matches(token) {
            //do stuff...what stuff??

            //then eat
            self.eat();
            println!("eat successfull");
            println!("=== END OUTPUT =======");
            Result::Ok(())
        } else {
            println!("Mismatch in check and eat!");
            println!("=== END OUTPUT =======");
            self.error_message(token)
        }
    }

    fn current_token(&self) -> C1Token {
        self.lexer.current_token().unwrap()
    }

    //eat konsumiert Token - aktueller Pointer und Lookahead je 1 weiter gesetzt
    // -> macht Lexer für einen
    fn eat(&mut self) {
        self.lexer.eat();
    }

    // Prüf ob das ihnen übergebene Token gleich dem aktuellen ist
    //gibt Ergebnis des Vergleichs zurück
    fn current_matches(&self, token: C1Token) -> bool {
        let current = self.lexer.current_token().unwrap();
        if current == token {
            return true;
        }
        false
    }

    // Prüf ob das ihnen übergebene Token gleich dem nächsten ist
    //gibt Ergebnis des Vergleichs zurück
    pub fn next_matches(&self, token: C1Token) -> bool {
        let next = self.lexer.peek_token().unwrap();
        println!("=== BEGIN NEXT MATCHES OUTPUT =======");
        println!("in next_matches");
        println!("next: {:?} ", next);
        println!("given: {:?} ", token);
        println!("=== END OUTPUT =======");
        if next == token {
            return true;
        }
        false
    }

    //Jede Regel wird eine Funktion
    fn program(&mut self, token: C1Token) -> ParseResult {
        println!("in program");
        // self.check_and_eat_token(token);
        // self.next_matches(token);
        //let res = self.function_definition(token);
        let mut res = Result::Ok(());
        let mut fail = false;

        while !fail && self.lexer.current_token().is_some() {
            res = self.function_definition(self.current_token());
            println!(
                "After loop in Program on line {:?}",
                self.lexer.current_line_number()
            );

            // println!(
            //     "Current token: {:?}, {:?}",
            //     self.current_token(),
            //     self.lexer.current_text()
            // );

            fail = res.is_err();
            println!("Fail is: {:?}", fail);
        }
        println!("Ending on line {:?}", self.lexer.current_line_number());
        // println!(
        //     "Current token: {:?}, {:?}",
        //     self.current_token(),
        //     self.lexer.current_text()
        // );
        res
    }

    fn function_definition(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN FUNC DEF OUTPUT =======");
        println!(
            "in function def, current token is: {:?}, {:?}, given was: {:?}",
            self.current_token(),
            self.lexer.current_text(),
            token
        );
        let intermediate_result = self
            .type_kw(token)
            .and(self.check_and_eat_token(C1Token::Identifier))
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
            .and(self.check_and_eat_token(C1Token::LeftBrace))
            .and(self.statementlist(self.current_token()));
        println!(
            "Current token after statementlist in function def: {:?}, {:?}",
            self.current_token(),
            self.lexer.current_text()
        );
        println!(
            "intermediate res in func def (before calling right brace): {:?}",
            intermediate_result
        );

        let res = self.check_and_eat_token(C1Token::RightBrace);
        // println!(
        //     "Current token eat right brace in function def: {:?}, {:?}",
        //     self.current_token(),
        //     self.lexer.current_text()
        // );
        println!("res in func def (aftercalling right brace): {:?}", res);
        return res;

        //Token mismatch here on RightBrace
    }

    fn function_call(&mut self, token: C1Token) -> ParseResult {
        println!("in function call");
        //if (self.current_matches(token))
        if token == C1Token::Identifier
            && self.lexer.peek_token().unwrap() == C1Token::LeftParenthesis
        {
            // self.check_and_eat_token(token);
            // self.check_and_eat_token(C1Token::LeftParenthesis);
            // self.check_and_eat_token(C1Token::RightParenthesis);
            // return Result::Ok(());
            return self
                .check_and_eat_token(token)
                .and(self.check_and_eat_token(C1Token::LeftParenthesis))
                .and(self.check_and_eat_token(C1Token::RightParenthesis));
        }
        self.error_message(token)
    }

    fn statementlist(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN STMT LIST OUTPUT =======");
        println!("==== END OUTPUT ======");
        //self.block(token).or(Result::Ok(()))
        let mut res = Result::Ok(());
        let mut fail = false;
        while !fail {
            res = self.block(token);
            if res.is_err() {
                println!("STMT LIST: FAIL TRUE!");
                fail = true;
            }
        }
        res //TODO: What to return here if error? Returns Error if not block, but could simply be something else?
    }

    fn block(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN  BLOCK OUTPUT =======");
        println!("in block");
        if token == C1Token::LeftBrace {
            println!("first Block Rule {{ tsmtlist }}");

            let mut res = self.check_and_eat_token(C1Token::LeftBrace);
            println!(
                "After res check and eat left Brace in first Block part, res is {:?}",
                res
            );
            res = self.statementlist(self.current_token());
            println!(
                "After res statementlist in first Block part, res is {:?}",
                res
            );
            res = self.check_and_eat_token(C1Token::RightBrace);
            println!(
                "After res Right Brace in first Block part, res is {:?}",
                res
            );
            println!("==== END BLOcK OUTPUT ======");
            return res;
        } else if token == C1Token::Identifier
            || token == C1Token::KwIf
            || token == C1Token::KwReturn
            || token == C1Token::KwPrintf
        {
            //TODO: Look here. Provides if, but in statement says identifier...?
            println!(
                "second Block Rule: statement. Token: {:?}, {:?}",
                self.current_token(),
                self.lexer.current_text()
            );
            //Error here
            println!("==== END BLOCK OUTPUT ======");
            self.statement(self.current_token())
        } else {
            println!("Error in block! Next token is {:?}", self.current_token());
            self.error_message(token)
        }
    }

    fn statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN STMT OUTPUT =======");
        println!("test");
        println!(
            "Given token: {:?}, current token: {:?}",
            token,
            self.current_token()
        );
        //let intermediate_result = self.if_statement(token);
        if token == C1Token::KwIf {
            println!("if Statement");
            println!("==== END OUTPUT ======");
            return self.if_statement(token);
        } else {
            println!("Token was not If: {:?}", token);
        }
        // } else {
        //     println!("****************************************************");
        //     println!("=== END OUTPUT =======");

        //     self.return_statement(self.current_token())
        //         .or(self.printf(self.current_token()))
        //         .or(self.stat_assignment(self.current_token()))
        //         .or(self.function_call(self.current_token()))
        //         .and(self.check_and_eat_token(C1Token::Semicolon))
        // }

        match token {
            C1Token::KwReturn => {
                println!("Was Return stmt");
                println!("==== END OUTPUT ======");
                self.return_statement(token)
            }
            C1Token::KwPrintf => {
                println!("Was PrintF");
                println!("==== END OUTPUT ======");
                self.printf(token)
                    .and(self.check_and_eat_token(C1Token::Semicolon))
            }
            C1Token::Identifier => {
                if self.next_matches(C1Token::Assign) {
                    // println!(
                    //     "Next token is {:?}, calling stat assign",
                    //     self.lexer.peek_token().unwrap()
                    // );
                    // let res = self.stat_assignment(token);
                    // //.and(self.check_and_eat_token(C1Token::Semicolon));
                    // println!("Res after stat assign in statement: {:?}", res);
                    // println!(
                    //     "Current token after res: {:?}, {:?}",
                    //     self.current_token(),
                    //     self.lexer.current_text()
                    // );
                    // println!("Trying to eat ;");
                    // println!("==== END OUTPUT ======");
                    // self.check_and_eat_token(C1Token::Semicolon);
                    self.call_statassign(token)
                } else if self.next_matches(C1Token::LeftParenthesis) {
                    println!(
                        "Next token is {:?}, calling function call",
                        self.lexer.peek_token().unwrap()
                    );
                    println!("==== END OUTPUT ======");
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

    fn call_statassign(&mut self, token: C1Token) -> ParseResult {
        println!(
            "Next token is {:?}, calling stat assign",
            self.lexer.peek_token().unwrap()
        );
        let res = self.stat_assignment(token);
        //.and(self.check_and_eat_token(C1Token::Semicolon));
        println!("Res after stat assign in statement: {:?}", res);
        println!(
            "Current token after res: {:?}, {:?}",
            self.current_token(),
            self.lexer.current_text()
        );
        println!("About to do res as ref?");
        res.as_ref()?;
        println!("Trying to eat ;");
        println!("==== END OUTPUT ======");
        self.check_and_eat_token(C1Token::Semicolon)
    }

    fn if_statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN IF STMT OUTPUT =======");
        println!(
            "token in if statement: {:?}, text: {:?}",
            token,
            self.lexer.current_text()
        );

        let res = self
            .check_and_eat_token(C1Token::KwIf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.assignment(self.current_token()));
        println!(
            "Back in if stmt after assignment. res: {:?}, current token: {:?}, {:?}",
            res,
            self.current_token(),
            self.lexer.current_text()
        );
        let next_rex = self
            .check_and_eat_token(C1Token::RightParenthesis)
            .and(self.block(self.current_token()));
        println!(
            "Back in if stmt after block. res: {:?}, current token: {:?}, {:?}",
            next_rex,
            self.current_token(),
            self.lexer.current_text()
        );
        println!("==== END IF STMT OUTPUT ======");
        return next_rex;
    }

    fn return_statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN RETURN OUTPUT =======");
        println!("in return");
        println!("==== END OUTPUT ======");
        let intermediate_result = self.check_and_eat_token(C1Token::KwReturn);
        println!(
            "intermediate result in return_statement: {:?}",
            intermediate_result
        );
        if self.assignment(self.lexer.current_token().unwrap()).is_ok() {
            println!("assign was ok in return_statement! ");
            self.check_and_eat_token(C1Token::Semicolon)
        } else {
            intermediate_result
        }
        //self.check_and_eat_token(C1Token::KwReturn)
    }

    fn printf(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN PRINT FOUTPUT =======");
        println!("==== END OUTPUT ======");

        let mut res = self
            .check_and_eat_token(C1Token::KwPrintf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis));
        println!(
            "printf: res after printf(: {:?}, current token {:?}, {:?}",
            res,
            self.current_token(),
            self.lexer.current_text()
        );

        res = self
            .assignment(self.current_token())
            .and(self.check_and_eat_token(C1Token::RightParenthesis));
        println!(
            "printf: res after assignment ): {:?}, current token {:?}, {:?}",
            res,
            self.current_token(),
            self.lexer.current_text()
        );
        println!("==== END PRINTF  OUTPUT ======");
        return res;
    }

    fn type_kw(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN TYPE KW OUTPUT =======");
        println!("in type kw");

        if token == C1Token::KwBoolean
            || token == C1Token::KwFloat
            || token == C1Token::KwInt
            || token == C1Token::KwVoid
        {
            println!(
                "Type KW: Token was one of Bool, Float, Int, Void: {:?}",
                token
            );
            println!("==== END TYPE KW OUTPUT ======");
            self.check_and_eat_token(token)
        } else {
            println!("Outputting error in type kw");
            println!("==== END TYPE KW OUTPUT ======");
            self.error_message(token)
        }
    }

    fn stat_assignment(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN STAT ASSIGN OUTPUT =======");
        println!("in stat assignement");
        println!("Token: {:?}, current: {:?}", token, self.current_token());
        println!("==== END STAT ASSIGN OUTPUT ======");

        let res = self
            .check_and_eat_token(C1Token::Identifier)
            .and(self.check_and_eat_token(C1Token::Assign))
            .and(self.assignment(self.current_token()));
        println!(
            "back in stat_assignment, res is {:?}, current token: {:?}, {:?}",
            res,
            self.current_token(),
            self.lexer.current_text()
        );
        return res;
    }

    fn assignment(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN ASSIGNMENT OUTPUT =======");
        println!("in assignment");
        println!(
            "token in assigment: {:?}, text: {:?}",
            token,
            self.lexer.current_text()
        );
        if token == C1Token::Identifier {
            println!("ID branch in assigment");
            if self.next_matches(C1Token::Assign) {
                println!(
                    "peek next token: is assign: {:?}, {:?}",
                    self.lexer.peek_token(),
                    self.lexer.peek_text()
                );
                println!("==== END ASSIGNMENT OUTPUT ======");
                self.check_and_eat_token(token)
                    .and(self.check_and_eat_token(C1Token::Assign))
                    .and(self.assignment(self.lexer.current_token().unwrap()))
            } else {
                println!(
                    "Token is ID, but next token not Assign, is: {:?}, {:?}",
                    self.lexer.peek_token(),
                    self.lexer.peek_text()
                );
                println!("Calling expr.");

                let res = self.expr(token);
                println!(
                    "back in assignment. res is: {:?}, current token is {:?}, {:?}",
                    res,
                    self.current_token(),
                    self.lexer.current_text()
                );
                println!("==== END ASSIGNMENT OUTPUT ======");
                return res;
            }
        } else {
            println!("Assignment: token not Id, calling expr: {:?}", token);
            let res = self.expr(token);
            println!(
                "back in assignment. res is: {:?}, current token is {:?}, {:?}",
                res,
                self.current_token(),
                self.lexer.current_text()
            );
            println!("==== END ASSIGNMENT OUTPUT ======");
            return res;
        }
    }

    fn expr(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN EXPR OUTPUT =======");
        println!(
            "current token: {:?}, given token: {:?}",
            self.current_token(),
            token
        );
        //let result = self.check_and_eat_token(token);
        let result = self.simpexpr(token);
        println!("returned to expr for first result.");
        println!("current token: {:?}", self.current_token());
        if self.current_matches(C1Token::Equal)
            || self.current_matches(C1Token::NotEqual)
            || self.current_matches(C1Token::LessEqual)
            || self.current_matches(C1Token::GreaterEqual)
            || self.current_matches(C1Token::Less)
            || self.current_matches(C1Token::Greater)
        {
            println!(
                "Token was one of == != <= >= < > : {:?}",
                self.current_token()
            );
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            println!("before going to simpexpr: {:?}", self.current_token());

            let res = self.simpexpr(self.current_token());
            println!(
                "Res after returning to expr: {:?}, token is {:?}, {:?}",
                res,
                self.current_token(),
                self.lexer.current_text()
            );
            println!("==== END EXPR OUTPUT ======");
            return res;
        } else {
            println!(
                "Token was NOT  one of == != <= >= < > : {:?}",
                self.current_token()
            );
            println!("==== END EXPR OUTPUT ======");
            result
        }
    }

    fn simpexpr(&mut self, token: C1Token) -> ParseResult {
        println!("in simpexpr");
        if self.current_matches(C1Token::Minus) {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
        }
        let mut res = self.term(self.current_token());
        println!(
            "Returned to simpexpr after intermediate result. Current token: {:?}",
            self.current_token()
        );

        //this whole block must be repeatable because of *
        let mut second_term = self.current_matches(C1Token::Plus)
            || self.current_matches(C1Token::Minus)
            || self.current_matches(C1Token::Or);
        // if self.current_matches(C1Token::Plus)
        //     || self.current_matches(C1Token::Minus)
        //     || self.current_matches(C1Token::Or)
        while second_term {
            println!(
                "staying in simpexpr, token was + - ||: {:?}",
                self.current_token()
            );
            let _ = self.check_and_eat_token(self.current_token());
            res = self.term(self.current_token());
            println!("Returned to simpexpr after token was + - ||");
            second_term = self.current_matches(C1Token::Plus)
                || self.current_matches(C1Token::Minus)
                || self.current_matches(C1Token::Or);
        }

        println!(
            "In Simpexpr, after returning. Token was not one of + - || , was: {:?}",
            self.current_token()
        );

        res
    }

    fn term(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN TERM OUTPUT =======");
        println!("in term");
        let intermediate_result = self.factor(token);
        println!("intermediate result: {:?}", intermediate_result);
        println!("Next token in term: {:?}", self.current_token());
        // let mut fail = intermediate_result.is_err();
        // while !fail {
        //     if self.current_matches(C1Token::Asterisk)
        //         || self.current_matches(C1Token::Slash)
        //         || self.current_matches(C1Token::And)
        //     {
        //         println!("Checking next token in Term; has to be one of * / &&");
        //         intermediate_result = self
        //             .check_and_eat_token(self.lexer.current_token().unwrap())
        //             .and(self.factor(self.lexer.current_token().unwrap()));
        //         fail = intermediate_result.is_err();
        //         //ENDLESS LOOP???
        //         //return self.factor(self.lexer.current_token().unwrap());
        //     }
        // }
        if self.current_matches(C1Token::Asterisk)
            || self.current_matches(C1Token::Slash)
            || self.current_matches(C1Token::And)
        {
            println!("Checking next token in Term; has to be one of * / &&");
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            return self.factor(self.lexer.current_token().unwrap());
        }
        println!("return intermediate result");
        println!("leaving term");
        intermediate_result
    }

    fn factor(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN FACTOR OUTPUT =======");
        println!("in factor");
        println!(
            "token in factor: {:?}, text: {:?}",
            token,
            self.lexer.current_text()
        );

        if token == C1Token::ConstInt
            || token == C1Token::ConstFloat
            || token == C1Token::ConstBoolean
        {
            println!("Token was Int, Float, Bool");
            println!("==== END OUTPUT ======");
            self.check_and_eat_token(token)
        } else if token == C1Token::Identifier {
            if self.next_matches(C1Token::LeftParenthesis) {
                println!(
                    "Id followed by LBrace: {:?}, {:?}",
                    self.current_token(),
                    self.lexer.peek_token().unwrap()
                );
                println!("Calling function call");
                println!("==== END OUTPUT ======");
                self.function_call(token)
            } else {
                println!(
                    "Id followed by NOT LBrace: {:?}, {:?}",
                    self.current_token(),
                    self.lexer.peek_token().unwrap()
                );
                println!("Calling Id eat");
                println!("==== END OUTPUT ======");
                self.check_and_eat_token(token)
            }
        } else if token == C1Token::LeftParenthesis {
            println!("In factor: Token was Left Parenthesis (");
            println!("==== END OUTPUT ======");
            let res = self
                .check_and_eat_token(token)
                .and(self.assignment(self.current_token()))
                .and(self.check_and_eat_token(C1Token::RightParenthesis));
            println!(
                "back in factor after calling assignment, current is {:?}, {:?}",
                self.current_token(),
                self.lexer.current_text()
            );
            return res;
        } else {
            //Result::Err(String::from("Factor: Token was not factor"))
            println!(
                "Not factor: {:?}, {:?}",
                self.current_token(),
                self.lexer.current_text()
            );
            Result::Err(String::from("Token was not factor"))
            //self.error_message(token)
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

    //TODO: erorr message with line number function
}

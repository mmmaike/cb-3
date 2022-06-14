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

    pub fn eof(&self, token: C1Token) -> bool {
        return true;
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
        self.function_definition(token)
    }

    fn function_definition(&mut self, token: C1Token) -> ParseResult {
        println!("in function def");
        self.type_kw(token)
            .and(self.check_and_eat_token(C1Token::Identifier))
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
            .and(self.check_and_eat_token(C1Token::LeftBrace))
            .and(self.statementlist(self.current_token()))
            .and(self.check_and_eat_token(C1Token::RightBrace))
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
            println!("==== END OUTPUT ======");
            self.check_and_eat_token(token)
                .and(self.statementlist(self.current_token()))
                .and(self.check_and_eat_token(C1Token::RightBrace))
        } else {
            println!("second Block Rule: statement");
            println!("==== END OUTPUT ======");
            self.statement(token)
        }
    }

    fn statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN STMT OUTPUT =======");
        println!("test");
        //let intermediate_result = self.if_statement(token);
        if token == C1Token::KwIf {
            println!("if Statement");
            println!("==== END OUTPUT ======");
            return self.if_statement(token);
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
            }
            C1Token::Identifier => {
                if self.next_matches(C1Token::Assign) {
                    println!(
                        "Next token is {:?}, calling stat assign",
                        self.lexer.peek_token().unwrap()
                    );
                    println!("==== END OUTPUT ======");
                    self.stat_assignment(token)
                        .and(self.check_and_eat_token(C1Token::Semicolon))
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

    fn if_statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN IF STMT OUTPUT =======");
        println!(
            "token in if statement: {:?}, text: {:?}",
            token,
            self.lexer.peek_token()
        );
        println!("==== END OUTPUT ======");
        self.check_and_eat_token(C1Token::KwIf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.assignment(self.lexer.current_token().unwrap()))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
            .and(self.block(self.lexer.current_token().unwrap()))
    }

    fn return_statement(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN RETURN OUTPUT =======");
        println!("in return");
        println!("==== END OUTPUT ======");
        let intermediate_result = self.check_and_eat_token(C1Token::KwReturn);
        if self.assignment(self.lexer.current_token().unwrap()).is_ok() {
            self.assignment(self.lexer.current_token().unwrap())
        } else {
            intermediate_result
        }
    }

    fn printf(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN PRINTFOUTPUT =======");
        println!("==== END OUTPUT ======");

        self.check_and_eat_token(C1Token::KwPrintf)
            .and(self.check_and_eat_token(C1Token::LeftParenthesis))
            .and(self.assignment(self.lexer.current_token().unwrap()))
            .and(self.check_and_eat_token(C1Token::RightParenthesis))
    }

    fn type_kw(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN TYPE KW OUTPUT =======");
        println!("in type kw");
        println!("==== END OUTPUT ======");

        if token == C1Token::KwBoolean
            || token == C1Token::KwFloat
            || token == C1Token::KwInt
            || token == C1Token::KwVoid
        {
            self.check_and_eat_token(token)
        } else {
            Result::Err(String::from("Token mismatch"))
        }
    }

    fn stat_assignment(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN STAT ASSIGN OUTPUT =======");
        println!("in stat assignement");
        println!("Token: {:?}, current: {:?}", token, self.current_token());
        println!("==== END OUTPUT ======");

        self.check_and_eat_token(C1Token::Identifier)
            .and(self.check_and_eat_token(C1Token::Assign))
            .and(self.assignment(self.current_token()))
    }

    fn assignment(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN ASSIGNMENT OUTPUT =======");
        println!("in assignement");
        println!(
            "token in assigment: {:?}, text: {:?}",
            token,
            self.lexer.current_text()
        );
        if token == C1Token::Identifier {
            println!("ID branch in assigment");
            println!("==== END OUTPUT ======");
            self.check_and_eat_token(token)
                .and(self.check_and_eat_token(C1Token::Assign))
                .and(self.assignment(self.lexer.current_token().unwrap()))
        } else {
            println!("==== END OUTPUT ======");
            self.expr(token)
        }
    }

    fn expr(&mut self, token: C1Token) -> ParseResult {
        println!("in expr");
        //let result = self.check_and_eat_token(token);
        let result = self.simpexpr(token);
        if self.current_matches(C1Token::Equal)
            || self.current_matches(C1Token::NotEqual)
            || self.current_matches(C1Token::LessEqual)
            || self.current_matches(C1Token::GreaterEqual)
            || self.current_matches(C1Token::Less)
            || self.current_matches(C1Token::Greater)
        {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            self.simpexpr(self.lexer.current_token().unwrap())
        } else {
            result
        }
    }

    fn simpexpr(&mut self, token: C1Token) -> ParseResult {
        println!("in simpexpr");
        if self.current_matches(C1Token::Minus) {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
        }
        let intermediate_result = self.term(self.lexer.current_token().unwrap());
        if self.current_matches(C1Token::Plus)
            || self.current_matches(C1Token::Minus)
            || self.current_matches(C1Token::Or)
        {
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            return self.term(self.lexer.current_token().unwrap());
        }

        intermediate_result
    }

    fn term(&mut self, token: C1Token) -> ParseResult {
        println!("=== BEGIN TERM OUTPUT =======");
        println!("in term");
        //TODO: recursive descent does not work here. Branch with more information (test for correct token)
        let intermediate_result = self.factor(token);
        println!("intermediate result: {:?}", intermediate_result);
        if self.current_matches(C1Token::Asterisk)
            || self.current_matches(C1Token::Slash)
            || self.current_matches(C1Token::And)
        {
            println!("Checking next token in Term; has to be one of * / &&");
            let _ = self.check_and_eat_token(self.lexer.current_token().unwrap());
            return self.factor(self.lexer.current_token().unwrap());
        }
        println!("return intermediate result");
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
            if self.next_matches(C1Token::LeftBrace) {
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
            println!("Token was Left Parenthesis (");
            println!("==== END OUTPUT ======");
            self.check_and_eat_token(token)
                .and(self.assignment(self.lexer.current_token().unwrap()))
                .and(self.check_and_eat_token(C1Token::RightParenthesis))
        } else {
            self.error_message(token)
        }
    }

    fn error_message(&mut self, token: C1Token) -> ParseResult {
        let msg = format!(
            "Token mismatch in line {:?} on token {:?}, expected {:?}",
            self.lexer.current_line_number().unwrap(),
            self.lexer.current_text().unwrap(),
            token
        );
        Result::Err(msg)
    }

    //TODO: erorr message with line number function
}

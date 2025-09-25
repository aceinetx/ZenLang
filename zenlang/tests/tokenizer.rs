use zenlang::tokenizer::*;

#[test]
fn tokenizer_test_number() {
    let mut tokenizer = Tokenizer::new("1.23 1".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.23)));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.)));
}

#[test]
fn tokenizer_test_strings() {
    let mut tokenizer = Tokenizer::new("\"Hello, World!\" \"Strings\"".into());
    if let Token::String(s) = tokenizer.next() {
        assert_eq!(s, "Hello, World!");
    } else {
        assert!(false);
    }
    if let Token::String(s) = tokenizer.next() {
        assert_eq!(s, "Strings");
    } else {
        assert!(false);
    }
}

#[test]
fn tokenizer_test_bool() {
    let mut tokenizer = Tokenizer::new("false true".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::False));
    let token = tokenizer.next();
    assert!(matches!(token, Token::True));
}

#[test]
fn tokenizer_test_fn() {
    let mut tokenizer = Tokenizer::new("fn     { }".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Fn));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lbrace));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Rbrace));
}

#[test]
fn tokenizer_test_identifier() {
    let mut tokenizer = Tokenizer::new("main dwa IDENT id_ent".into());
    if let Token::Identifier(s) = tokenizer.next() {
        assert_eq!(s, "main");
    } else {
        assert!(false);
    }
    if let Token::Identifier(s) = tokenizer.next() {
        assert_eq!(s, "dwa");
    } else {
        assert!(false);
    }
    if let Token::Identifier(s) = tokenizer.next() {
        assert_eq!(s, "IDENT");
    } else {
        assert!(false);
    }
    if let Token::Identifier(s) = tokenizer.next() {
        assert_eq!(s, "id_ent");
    } else {
        assert!(false);
    }
}

#[test]
fn tokenizer_test_get_line() {
    let mut tokenizer = Tokenizer::new("fn main\n{\n}".into());
    let _token = tokenizer.next();
    assert_eq!(tokenizer.get_line(), 1);
    let _token = tokenizer.next();
    assert_eq!(tokenizer.get_line(), 1);
    let _token = tokenizer.next();
    assert_eq!(tokenizer.get_line(), 2);
    let _token = tokenizer.next();
    assert_eq!(tokenizer.get_line(), 3);
}

#[test]
fn tokenizer_test_null() {
    let mut tokenizer = Tokenizer::new("null null".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Null));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Null));
}

#[test]
fn tokenizer_test_return() {
    let mut tokenizer = Tokenizer::new("return return".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Return));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Return));
}

#[test]
fn tokenizer_test_brace() {
    let mut tokenizer = Tokenizer::new("{}{".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lbrace));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Rbrace));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lbrace));
}

#[test]
fn tokenizer_test_paren() {
    let mut tokenizer = Tokenizer::new("()(".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lparen));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Rparen));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lparen));
}

#[test]
fn tokenizer_test_bracket() {
    let mut tokenizer = Tokenizer::new("[][".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lbracket));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Rbracket));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Lbracket));
}

#[test]
fn tokenizer_test_comma() {
    let mut tokenizer = Tokenizer::new(",,".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Comma));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Comma));
}

#[test]
fn tokenizer_test_eq() {
    let mut tokenizer = Tokenizer::new("==".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('=', '=')));
}

#[test]
fn tokenizer_test_semicolon() {
    let mut tokenizer = Tokenizer::new(";;;".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Semicolon));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Semicolon));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Semicolon));
}

#[test]
fn tokenizer_test_assign() {
    let mut tokenizer = Tokenizer::new("= =".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Assign));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Assign));
}

#[test]
fn tokenizer_test_eof() {
    let mut tokenizer = Tokenizer::new("=fn".into());
    let token = tokenizer.next();
    assert!(matches!(token, Token::Assign));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Fn));
    let token = tokenizer.next();
    assert!(matches!(token, Token::EOF));
    let token = tokenizer.next();
    assert!(matches!(token, Token::EOF));
}

#[test]
fn tokenizer_test_eq_2() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 == 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('=', '=')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_lt() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 < 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('<', '<')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_gt() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 > 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('>', '>')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_le() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 <= 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('<', '=')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_ge() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 >= 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('>', '=')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_neq() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 != 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::OperatorCmp('!', '=')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_not() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 ! 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::Operator('!')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_bitand() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 & 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::BitOperator('&', '&')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_bitor() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 | 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::BitOperator('|', '|')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_bitshr() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 >> 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::BitOperator('>', '>')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_bitshl() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 << 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::BitOperator('<', '<')));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_while() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 while 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::While));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_break() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 break 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::Break));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_escape_str() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 \"Hello\\n\" 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    if let Token::String(s) = token {
        assert_eq!(s, "Hello\n");
    } else {
        assert!(false);
    }
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

#[test]
fn tokenizer_test_dot() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 . 1 + 2; }".into());
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    tokenizer.next();
    let token = tokenizer.next();
    assert!(matches!(token, Token::Dot));
    let token = tokenizer.next();
    assert!(matches!(token, Token::Number(1.0)));
}

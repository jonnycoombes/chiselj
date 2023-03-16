extern crate test;

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::{env, io};
use test::Bencher;

use log::error;

use chisel_utils::string_table::StringTable;

use crate::json::lexer::{Lexer, PackedToken, Token};
use crate::parser_coords::ParserCoords;
use crate::parser_errors::{ParserError, ParserResult};

#[test]
fn should_parse_basic_tokens() {
    let buffer: &[u8] = "{}[],:".as_bytes();
    let mut reader = BufReader::new(buffer);
    let table = Rc::new(RefCell::new(StringTable::new()));
    let mut lexer = Lexer::new(table, &mut reader);
    let mut tokens: Vec<Token> = vec![];
    let mut coords: Vec<(ParserCoords, Option<ParserCoords>)> = vec![];
    for _ in 1..=7 {
        let token = lexer.consume().unwrap();
        tokens.push(token.token);
        coords.push((token.start, token.end));
    }
    assert_eq!(
        tokens,
        [
            Token::StartObject,
            Token::EndObject,
            Token::StartArray,
            Token::EndArray,
            Token::Comma,
            Token::Colon,
            Token::EndOfInput
        ]
    );
}

#[test]
fn should_parse_null_and_booleans() {
    let buffer: &[u8] = "null true    falsetruefalse".as_bytes();
    let mut reader = BufReader::new(buffer);
    let table = Rc::new(RefCell::new(StringTable::new()));
    let mut lexer = Lexer::new(table, &mut reader);
    let mut tokens: Vec<Token> = vec![];
    let mut coords: Vec<(ParserCoords, Option<ParserCoords>)> = vec![];
    for _ in 1..=6 {
        let token = lexer.consume().unwrap();
        tokens.push(token.token);
        coords.push((token.start, token.end));
    }
    assert_eq!(
        tokens,
        [
            Token::Null,
            Token::Bool(true),
            Token::Bool(false),
            Token::Bool(true),
            Token::Bool(false),
            Token::EndOfInput
        ]
    );
}

#[test]
fn should_parse_strings() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/utf-8/strings.txt");
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines();
    for l in lines.flatten() {
        if !l.is_empty() {
            let mut reader = BufReader::new(l.as_bytes());
            let table = Rc::new(RefCell::new(StringTable::new()));
            let mut lexer = Lexer::new(table, &mut reader);
            let token = lexer.consume().unwrap();
            assert_eq!(token.token, Token::Str(l))
        }
    }
}

#[test]
fn should_correctly_identity_dodgy_strings() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/utf-8/dodgy_strings.txt");
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines();
    for l in lines.flatten() {
        if !l.is_empty() {
            let mut reader = BufReader::new(l.as_bytes());
            let table = Rc::new(RefCell::new(StringTable::new()));
            let mut lexer = Lexer::new(table, &mut reader);
            let mut error_token: Option<ParserError> = None;
            loop {
                let token = lexer.consume();
                match token {
                    Ok(packed) => {
                        if packed.token == Token::EndOfInput {
                            break;
                        }
                    }
                    Err(err) => {
                        error_token = Some(err.clone());
                        println!(
                            "Dodgy string found: '{}' -> {} : {}",
                            l,
                            err.message,
                            err.coords.unwrap()
                        );
                        break;
                    }
                }
            }
            assert!(error_token.is_some());
        }
    }
}

#[bench]
fn string_parsing_benchmark(bencher: &mut Bencher) {
    bencher.iter(|| {
        should_parse_strings();
    })
}

#[test]
fn should_correctly_report_errors_for_booleans() {
    let buffer: &[u8] = "true farse".as_bytes();
    let mut reader = BufReader::new(buffer);
    let table = Rc::new(RefCell::new(StringTable::new()));
    let mut lexer = Lexer::new(table, &mut reader);
    let mut results: Vec<ParserResult<PackedToken>> = vec![];
    for _ in 1..=2 {
        results.push(lexer.consume());
    }
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    println!("Parse error: {:?}", results[1]);
}

extern crate test;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Once;
use std::time::Instant;
use test::Bencher;

use log::trace;

use crate::parser_errors::ParserErrorCode;
use crate::utils::char_stream::CharStream;

#[serial]
#[test]
fn can_create_from_array() {
    let buffer: &[u8] = &[0x10, 0x12, 0x23, 0x12];
    let mut reader = BufReader::new(buffer);
    let _stream = CharStream::new(&mut reader);
}

#[test]
fn can_create_from_file() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/utf-8/fuzz.txt");
    let f = File::open(path);
    let mut reader = BufReader::new(f.unwrap());
    let _stream = CharStream::new(&mut reader);
}

#[serial]
#[test]
fn pass_a_fuzz_test() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/utf-8/fuzz.txt");
    let f = File::open(path);
    let mut reader = BufReader::new(f.unwrap());
    let mut stream = CharStream::new(&mut reader);
    let start = Instant::now();
    while stream.next_char().is_ok() {}
    println!("Consumed all UTF-8 in {:?}", start.elapsed());
}

#[serial]
#[test]
fn process_a_simple_file() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/json/simple_structure.json");
    let f = File::open(path);
    let mut reader = BufReader::new(f.unwrap());
    let mut stream = CharStream::new(&mut reader);
    let start = Instant::now();
    while stream.next_char().is_ok() {}
    println!("Consumed all UTF-8 in {:?}", start.elapsed());
}

#[bench]
fn simple_file_benchmark(bencher: &mut Bencher) {
    bencher.iter(|| {
        process_a_simple_file();
    })
}

#[bench]
fn complex_file_benchmark(bencher: &mut Bencher) {
    bencher.iter(|| {
        process_a_complex_file();
    })
}

#[serial]
#[test]
fn process_a_complex_file() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/json/twitter.json");
    let f = File::open(path);
    let mut reader = BufReader::new(f.unwrap());
    let mut stream = CharStream::new(&mut reader);
    let start = Instant::now();
    while stream.next_char().is_ok() {}
    println!("Consumed all UTF-8 in {:?}", start.elapsed());
}

#[serial]
#[test]
fn process_a_large_file() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/json/events.json");
    let f = File::open(path);
    let mut reader = BufReader::with_capacity(4096, f.unwrap());
    let mut stream = CharStream::new(&mut reader);
    let start = Instant::now();
    loop {
        let result = stream.next_char();
        if result.is_err() {
            break;
        }
    }
    println!("Consumed all UTF-8 in {:?}", start.elapsed());
}

#[test]
fn should_correctly_decode_utf8_characters() {
    let buffer: &[u8] = "ह".as_bytes();
    let mut reader = BufReader::new(buffer);
    let mut stream = CharStream::new(&mut reader);
    let char = stream.next().unwrap();
    assert_eq!(char, 'ह')
}

#[serial]
#[test]
fn should_be_an_iterator() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/json/events.json");
    let f = File::open(path);
    let mut reader = BufReader::with_capacity(4096, f.unwrap());
    let stream = CharStream::new(&mut reader);
    let start = Instant::now();
    for _c in stream {}
    println!("Consumed all UTF-8 in {:?}", start.elapsed());
}

#[serial]
#[test]
fn should_produce_eof_markers() {
    let path = env::current_dir()
        .unwrap()
        .join("fixtures/samples/json/events.json");
    let f = File::open(path);
    let mut reader = BufReader::with_capacity(16384, f.unwrap());
    let mut stream = CharStream::new(&mut reader);
    loop {
        let result = stream.next_char();
        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
                match err.code {
                    ParserErrorCode::EndOfInput => {
                        break;
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}

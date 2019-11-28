extern crate wasm_bindgen_test;

#[macro_use]
mod utils;

use serde::Serialize;
use wasm_bindgen_test::{
    wasm_bindgen_test,
    wasm_bindgen_test_configure,
};

use utils::parse_test_example;
use vyper_parser::string_utils::StringPositions;
use vyper_parser::tokenizer::{
    tokenize,
    Token,
    TokenType,
};

wasm_bindgen_test_configure!(run_in_browser);

/// A python token object similar to those defined in python's stdlib `tokenize`
/// module.
#[derive(Serialize)]
struct PythonTokenInfo<'a> {
    pub typ: TokenType,
    pub string: &'a str,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub line: &'a str,
}

impl<'a> PythonTokenInfo<'a> {
    pub fn from_token_and_positions(
        tok: &'a Token<'a>,
        string_pos: &mut StringPositions<'_>,
    ) -> Self {
        let start_pos = match string_pos.get_pos(tok.span.start) {
            Some(pos) => pos,
            None => string_pos.get_eof(),
        };
        let end_pos = match string_pos.get_pos(tok.span.end) {
            Some(pos) => pos,
            None => string_pos.get_eof(),
        };

        Self {
            typ: tok.typ,
            string: tok.string,
            start: (start_pos.line, start_pos.col),
            end: (end_pos.line, end_pos.col),
            line: tok.line,
        }
    }
}

fn get_rust_token_json(input: &str) -> String {
    let tokens = tokenize(input).unwrap();
    let mut string_pos = StringPositions::new(input);

    // Convert vyper tokens into python tokens
    let python_tokens = tokens
        .iter()
        .map(|tok| PythonTokenInfo::from_token_and_positions(tok, &mut string_pos))
        .collect::<Vec<PythonTokenInfo>>();

    serde_json::to_string_pretty(&python_tokens).unwrap()
}

fn assert_fixture_is_valid(filename: &str, input: &str, expected_ser: &str) {
    let actual_ser = get_rust_token_json(input);

    assert_strings_eq!(
        actual_ser,
        expected_ser,
        "\nTokenizations did not match for {}",
        filename,
    );
}

#[test]
#[wasm_bindgen_test]
fn test_tokenize() {
    do_with_fixtures!(
        assert_fixture_is_valid,
        "fixtures/tokenizer/basic.py.json",
        "fixtures/tokenizer/triple_quote_strings.py.json",
        "fixtures/tokenizer/single_quote_strings.py.json",
        "fixtures/tokenizer/continued_statements.py.json",
        "fixtures/tokenizer/validator_registration.v.py.json",
        "fixtures/tokenizer/tokenize.py.json",
    );
}

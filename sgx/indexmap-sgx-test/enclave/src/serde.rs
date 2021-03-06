//#![cfg(feature = "serde-1")]

//#[macro_use]
//extern crate indexmap;
//extern crate serde_test;
//extern crate fnv;

use serde_test::{Token, assert_tokens};

//#[test]
pub fn test_serde() {
    let map = indexmap! { 1 => 2, 3 => 4 };
    assert_tokens(&map,
                  &[Token::Map { len: Some(2) },
                    Token::I32(1),
                    Token::I32(2),
                    Token::I32(3),
                    Token::I32(4),
                    Token::MapEnd]);
}

//#[test]
pub fn test_serde_set() {
    let set = indexset! { 1, 2, 3, 4 };
    assert_tokens(&set,
                  &[Token::Seq { len: Some(4) },
                    Token::I32(1),
                    Token::I32(2),
                    Token::I32(3),
                    Token::I32(4),
                    Token::SeqEnd]);
}

//#[test]
pub fn test_serde_fnv_hasher() {
    let mut map: ::indexmap::IndexMap<i32, i32, ::fnv::FnvBuildHasher> = Default::default();
    map.insert(1, 2);
    map.insert(3, 4);
    assert_tokens(&map,
                  &[Token::Map { len: Some(2) },
                    Token::I32(1),
                    Token::I32(2),
                    Token::I32(3),
                    Token::I32(4),
                    Token::MapEnd]);
}

//#[test]
pub fn test_serde_map_fnv_hasher() {
    let mut set: ::indexmap::IndexSet<i32, ::fnv::FnvBuildHasher> = Default::default();
    set.extend(1..5);
    assert_tokens(&set,
                  &[Token::Seq { len: Some(4) },
                    Token::I32(1),
                    Token::I32(2),
                    Token::I32(3),
                    Token::I32(4),
                    Token::SeqEnd]);
}

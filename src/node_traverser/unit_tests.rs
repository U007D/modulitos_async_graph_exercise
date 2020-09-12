#![allow(non_snake_case)]
use super::*;
use assert2::assert;
use serde::Deserialize;
use serde_json::{self, from_str};
use crate::adapters::clients::MemoryClient;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Node {
    children: Vec<String>,
    reward: u32,
}

#[test]
fn get__node_a_yields_expected_result() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let expected_res = from_str::<Node>(
        r#"
        {
            "children": [ "b", "e" ],
            "reward": 1
        }
        "#,
    )?;
    let client = MemoryClient::new();
    let traverser = NodeTraverser::new(client);
    let sut = move |node| traverser.get(node);

    // When
    let res = sut("a")?;

    // Then
    assert!(from_str::<Node>(&res)? == expected_res);
    Ok(())
}

#![allow(non_snake_case)]
use super::*;
use crate::{adapters::clients::MemoryClient, error};
use assert2::assert;
use serde_json::{self, from_str};
use futures::stream::FuturesUnordered;
use futures::{TryStreamExt};

#[async_std::test]
async fn get__node_a_yields_expected_result() -> Result<(), error::client::Error> {
    // Given
    let expected_res = from_str::<Node>(
        r#"
        {
            "children": [ "b", "e" ],
            "reward": 1
        }
        "#,
    )?;
    let traverser = NodeTraverser::new(MemoryClient::new());

    // When
    let res = traverser.get("a").await?;

    // Then
    assert!(res == expected_res);
    Ok(())
}

#[async_std::test]
async fn get__returns_results_asynchronously() -> Result<(), error::client::Error> {
    // Given
    let expected_ress = vec![
        from_str::<Node>(
            r#"
            {
                "children": [ "b", "e" ],
                "reward": 1
            }
            "#,
        )?,
        from_str::<Node>(
            r#"
            {
                "children": ["f"],
                "reward": 4
            }
        "#,
        )?,
        from_str::<Node>(
            r#"
            {
                "children": [ "c" ],
                "reward": 2
            }
            "#,
        )?,
    ];

    // Note `MemoryClient` is explicitly designed to return node b in 100ms with the other nodes
    // returning immediately.  Request order a, b then e should resolve in order a, e then b.
    let nodes = vec!["a", "b", "e"];
    let traverser = NodeTraverser::new(MemoryClient::new());
    let futs = FuturesUnordered::new();

    // When
    nodes.into_iter().for_each(|node| futs.push(traverser.get(node)));

    // Then
    futs.try_collect::<Vec<_>>().await?
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .zip(expected_ress)
        .all(|(actual, expected)| {
            assert!(actual == expected);
            true
        });
    Ok(())
}

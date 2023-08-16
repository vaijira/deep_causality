// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

#![forbid(unsafe_code)]

use crate::prelude::{StorageMatrixGraph, UltraGraph};

pub mod prelude;
pub mod protocols;
pub mod errors;
pub mod storage;
pub mod types;

/// Returns a new UltraGraph with matrix storage backend.
///
/// # Arguments
/// * Capacity refers to the maximum number of nodes that fit into the graph before a resize occurs.
///
/// # Example:
/// ```
/// use ultragraph::prelude::*;
///
/// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
/// pub struct Data {
///     x: u8,
/// }
///
///  let mut g = ultragraph::new_with_matrix_storage::<Data>(10);
///  assert!(g.is_empty());
///
///  let d = Data { x: 1 };
///  let root_index = g.add_root_node(d);
///  assert_eq!(root_index, 0);
///
///  let d = Data { x: 42 };
///  let node_a_index = g.add_node(d);
///  assert_eq!(node_a_index, 1);
///
///  let data = g.get_node(1).unwrap();
///  assert_eq!(data.x, d.x);
///
///  let res = g.add_edge(root_index, node_a_index);
///  assert!(res.is_ok());
/// ```
pub fn new_with_matrix_storage<T>(
    capacity: usize
)
    -> UltraGraph<StorageMatrixGraph<T>, T>
{
    UltraGraph::new(StorageMatrixGraph::<T>::new_with_capacity(capacity))
}

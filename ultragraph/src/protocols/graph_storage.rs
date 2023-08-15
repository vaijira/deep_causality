// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use crate::protocols::graph_like::GraphLike;
use crate::protocols::graph_root::GraphRoot;

pub trait GraphStorage<T>: GraphLike<T> + GraphRoot<T>
    where
        T: Copy
{
    fn size(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn number_nodes(&self) -> usize;

    fn number_edges(&self) -> usize;
}
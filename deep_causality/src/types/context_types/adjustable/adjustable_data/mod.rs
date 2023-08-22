use deep_causality_macros::{Constructor, Getters};

// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.
use crate::prelude::Datable;

mod adjustable;
mod display;
mod identifiable;

#[derive(Getters, Constructor, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AdjustableData<T>
    where T: Copy + Default,
{
    #[getter(name = data_id)] // Rename ID getter to prevent conflict impl with identifiable
    id: u64,
    data: T,
}

// Type tag required for context.
impl<T> Datable for AdjustableData<T> where T: Copy + Default {}
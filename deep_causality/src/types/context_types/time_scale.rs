// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum TimeScale
{
    #[default]
    NoScale,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl Display for TimeScale
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

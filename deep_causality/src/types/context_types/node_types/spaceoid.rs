// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.
use std::fmt::{Display, Formatter};

use deep_causality_macros::Constructor;

use crate::prelude::{Identifiable, Spatial};

#[derive(Constructor, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Spaceoid
{
    id: u64,
    x: i64,
    y: i64,
    z: i64,
}


impl Identifiable for Spaceoid
{
    fn id(&self) -> u64 {
        self.id
    }
}


impl Spatial for Spaceoid
{
    fn x(&self) -> i64 {
        self.x
    }

    fn y(&self) -> i64 {
        self.y
    }

    fn z(&self) -> i64 {
        self.z
    }
}


impl Display for Spaceoid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spaceoid: id={}, x={}, y={}, z={}",
               self.id, self.x, self.y, self.z
        )
    }
}

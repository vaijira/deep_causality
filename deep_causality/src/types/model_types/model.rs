// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.


use crate::prelude::{Assumption, Causaloid};
use crate::protocols::identifiable::Identifiable;

#[derive(Clone, Copy)]
pub struct Model<'l> {
    id: u64,
    author: &'l str,
    description: &'l str,
    assumptions: &'l Option<&'l Vec<&'l Assumption>>,
    causaloid: &'l Causaloid<'l>,
}

impl<'l> Model<'l> {
    pub fn new(id: u64, author: &'l str, description: &'l str, assumptions: &'l Option<&'l Vec<&'l Assumption>>, model: &'l Causaloid) -> Self {
        Self { id, author, description, assumptions, causaloid: model }
    }
}

impl<'l> Model<'l> {
    pub fn author(&self) -> &'l str {
        self.author
    }
    pub fn description(&self) -> &'l str {
        self.description
    }
    pub fn assumptions(&self) -> &'l Option<&'l Vec<&'l Assumption>> {
        self.assumptions
    }
    pub fn causaloid(&self) -> &'l Causaloid {
        self.causaloid
    }
}

impl<'l> Identifiable for Model<'l> {
    fn id(&self) -> u64 {
        self.id
    }
}
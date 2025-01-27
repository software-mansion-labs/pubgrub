// SPDX-License-Identifier: MPL-2.0

//! Non exposed modules.

mod arena;
mod core;
mod incompatibility;
mod partial_solution;
mod small_map;
mod small_vec;

pub use core::State;
pub use incompatibility::Incompatibility;
pub(crate) use arena::{Arena, Id};
pub(crate) use incompatibility::{IncompDpId, IncompId, Relation};
pub(crate) use partial_solution::{DecisionLevel, PartialSolution, SatisfierSearch};
pub(crate) use small_map::SmallMap;
pub(crate) use small_vec::SmallVec;

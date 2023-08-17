use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

use crate::utils::some_if;

use super::item_spec::ItemSpecRef;
use super::{prices, ItemQuality as IQ, ItemRef};

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumCount, EnumIter, PartialEq)]
pub enum Substance {
    Arcane,
    Explosive,
    Ore,
    Solvent,
}

impl Substance {
    pub fn to_svg(self) -> &'static str {
        use crate::svg;

        match self {
            Self::Ore => svg::STONE,
            Self::Explosive => svg::DYNAMITE,
            Self::Solvent => svg::DROP,
            Self::Arcane => svg::PENTAGRAM,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Reagent([u8; Substance::COUNT]);

impl Reagent {
    pub fn iter(&self) -> impl Iterator<Item = (Substance, u8)> + '_ {
        Substance::iter().zip(self.0.iter().copied())
    }
    pub fn get(&self, sub: Substance) -> Option<u8> {
        let x = self.0[sub as usize];
        some_if(x != 0).map(|_| x)
    }
}

struct ReagentBuilder([u8; Substance::COUNT]);

impl ReagentBuilder {
    const fn new() -> Self {
        Self([0; Substance::COUNT])
    }
    const fn solvent(mut self) -> Self {
        self.0[Substance::Solvent as usize] = 1;
        self
    }
    const fn arcane(mut self) -> Self {
        self.0[Substance::Arcane as usize] = 1;
        self
    }
    const fn explosive(mut self) -> Self {
        self.0[Substance::Explosive as usize] = 1;
        self
    }
    const fn build(self, quality: IQ) -> Reagent {
        let amount = (quality as u8 + 1) * 5;
        Reagent(multiply_by(amount, 0, self.0))
    }
}

const fn multiply_by(am: u8, i: usize, mut arr: [u8; Substance::COUNT]) -> [u8; Substance::COUNT] {
    if i == arr.len() {
        arr
    } else {
        arr[i] *= am;
        multiply_by(am, i + 1, arr)
    }
}

#[rustfmt::skip]
pub mod flora {
    use super::*;

    const fn flora(name: &'static str, quality: IQ, reagent: ReagentBuilder) -> ItemRef {
        ItemRef {
            name,
            specs: ItemSpecRef::Reagent(reagent.build(quality)),
            price: prices::FLORA[quality as usize],
            is_bulky: false,
            quality,
            stacks: Some(5),
        }
    }

    pub const SAGEROOT: ItemRef = flora("sageroot", IQ::Common, ReagentBuilder::new().arcane().solvent());
    pub const FUNGI: ItemRef = flora("black shroom", IQ::Uncommon, ReagentBuilder::new().solvent().explosive());
    pub const MOON_FLOWER: ItemRef = flora("moon flower", IQ::Wondrous, ReagentBuilder::new().arcane().solvent());
}

pub const ALL: [&ItemRef; 3] = [&flora::FUNGI, &flora::SAGEROOT, &flora::MOON_FLOWER];

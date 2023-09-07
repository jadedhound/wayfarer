use super::{ItemPropRef as Prop, ItemRef};

const HARDTACK: ItemRef = ItemRef::new("hard tack", 1, &[Prop::Edible(1)]);

pub(super) const ALL: [&ItemRef; 1] = [&HARDTACK];

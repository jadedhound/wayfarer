use super::item_prop::ItemPropRef as Prop;
use super::ItemRef;

pub const ERROR_ITEM: ItemRef = ItemRef::new("error", 0, &[]);
pub const FATIGUE: ItemRef = ItemRef::new("fatigue", 0, &[]);
const LOOT: ItemRef = ItemRef::new("loot", 0, &[]);
const LOOT_HEAVY: ItemRef = ItemRef::new("loot heavy", 0, &[Prop::Bulky]);

pub const ITEMS: [&ItemRef; 4] = [&ERROR_ITEM, &FATIGUE, &LOOT, &LOOT_HEAVY];

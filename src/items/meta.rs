use super::ItemRef;

pub const ERROR_ITEM: ItemRef = ItemRef::simple("error", 0);
pub const EMPTY_ITEM: ItemRef = ItemRef::simple("", 0);
const LOOT_COMMON: ItemRef = ItemRef::simple("loot (common)", 100);
const LOOT_UNCOMMON: ItemRef = ItemRef::simple("loot (uncommon)", 250);
const LOOT_RARE: ItemRef = ItemRef::simple("loot (rare)", 500);
const LOOT_WONDROUS: ItemRef = ItemRef::simple("loot (wondrous)", 1000);
const LOOT_MYTHICAL: ItemRef = ItemRef::simple("loot (mythical)", 2000);

pub const ITEMS: [&ItemRef; 5] = [
    &LOOT_COMMON,
    &LOOT_UNCOMMON,
    &LOOT_RARE,
    &LOOT_WONDROUS,
    &LOOT_MYTHICAL,
];

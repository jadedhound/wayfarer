use super::item_spec::ItemSpecRef;
use super::{ItemQuality, ItemRef};

const fn simple(name: &'static str, price: u32) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Simple,
        weight: 1,
        price,
        quality: ItemQuality::Common,
        stacks: None,
    }
}

// SUNDRY

const ROPE: ItemRef = simple("50 ft. rope", 10);
const IRON_CHAIN: ItemRef = simple("10 ft. iron chain", 10);
const MANACLES: ItemRef = simple("iron manacles", 10);
const LANTERN: ItemRef = simple("hooded lantern", 50);
const MARBLES: ItemRef = simple("sack of marbles", 10);
const TENT: ItemRef = simple("personal tent", 50);
const SACK: ItemRef = simple("sack", 1);
const CROWBAR: ItemRef = simple("crowbar", 10);
const HAMMER: ItemRef = simple("hammer", 50);
const CHISEL: ItemRef = simple("chisel", 10);
const LOCKPICK: ItemRef = simple("lockpicks", 100);
const HORN: ItemRef = simple("horn", 10);
const NEEDLE: ItemRef = simple("needle", 10);
const THREAD: ItemRef = simple("thread", 10);
const SHOVEL: ItemRef = simple("shovel", 10);
const HOURGLASS: ItemRef = simple("hourglass", 300);
const MIRROR: ItemRef = simple("mirror", 200);
const FISHNET: ItemRef = simple("fishing net", 10);
const GRAPPLING_HOOK: ItemRef = simple("grappling hook", 10);
const BELL: ItemRef = simple("bell", 20);
const IRON_FILE: ItemRef = simple("iron file", 10);
const BUCKET: ItemRef = simple("bucket", 5);

// META

pub const ERROR_ITEM: ItemRef = simple("Error", 0);
pub const FATIGUE: ItemRef = simple("Fatigue", 0);

pub(super) const ITEMS_SIMPLE: [&ItemRef; 22] = [
    &ROPE,
    &IRON_CHAIN,
    &MANACLES,
    &LANTERN,
    &MARBLES,
    &TENT,
    &SACK,
    &CROWBAR,
    &HAMMER,
    &CHISEL,
    &LOCKPICK,
    &HORN,
    &NEEDLE,
    &THREAD,
    &SHOVEL,
    &HOURGLASS,
    &MIRROR,
    &FISHNET,
    &GRAPPLING_HOOK,
    &BELL,
    &IRON_FILE,
    &BUCKET,
];

/// Starting adventuring items.
pub const SUNDRIES: [&ItemRef; 22] = [
    &ROPE,
    &IRON_CHAIN,
    &MANACLES,
    &LANTERN,
    &MARBLES,
    &TENT,
    &SACK,
    &CROWBAR,
    &HAMMER,
    &CHISEL,
    &LOCKPICK,
    &HORN,
    &NEEDLE,
    &THREAD,
    &SHOVEL,
    &HOURGLASS,
    &MIRROR,
    &FISHNET,
    &GRAPPLING_HOOK,
    &BELL,
    &IRON_FILE,
    &BUCKET,
];
pub const ITEMS_META: [&ItemRef; 1] = [&FATIGUE];

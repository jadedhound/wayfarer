use super::{ItemPropRef as Prop, ItemRef};
use crate::buffs::{BuffPropRef as BuffProp, BuffRef};
use crate::pc::Ability;
use crate::utils::counter::Counter;
use crate::utils::turns::Turns;

// ---- B ----
const BEAR_TRAP: ItemRef = ItemRef::new(
    "bear trap",
    20,
    &[Prop::Usable(
        "once set, a creature that triggers the trap takes 1d8 damage",
    )],
);
const BEDROLL: ItemRef = ItemRef::simple("bedroll", 10);
const BELL: ItemRef = ItemRef::simple("bell (small)", 20);
const BELLOWS: ItemRef = ItemRef::simple("bellows", 10);
const BLOCK_AND_TACKLE: ItemRef = ItemRef::simple("block and tackle", 30);
const BOTTLE: ItemRef = ItemRef::simple("bottle", 1);
const BUCKET: ItemRef = ItemRef::simple("bucket", 5);
// ---- C ----
const CARDS: ItemRef = ItemRef::simple("cards (pack)", 5);
const CAULDRON: ItemRef = ItemRef::simple("cauldron", 10);
const CHALK: ItemRef = ItemRef::new("chalk", 5, &[Prop::Count(Counter::new(5))]);
const CHISEL: ItemRef = ItemRef::simple("chisel", 10);
const CROWBAR: ItemRef = ItemRef::simple("crowbar", 10);
// ---- F ----
const FACE_PAINT: ItemRef = ItemRef::simple("face paint", 10);
const FISHING_NET: ItemRef = ItemRef::simple("fishing net", 10);
const FISHING_ROD_AND_TACKLE: ItemRef = ItemRef::simple("fishing rod and tackle", 10);
// ---- G ----
const GLUE: ItemRef = ItemRef::simple("glue", 1);
const GRAPPLING_HOOK: ItemRef = ItemRef::simple("grappling hook", 10);
// ---- H ----
const HAND_DRILL: ItemRef = ItemRef::simple("hand drill", 10);
const HAMMER: ItemRef = ItemRef::simple("hammer", 50);
const HORN: ItemRef = ItemRef::simple("horn", 10);
// ---- I ----
const IRON_CHAIN: ItemRef = ItemRef::simple("10 ft. iron chain", 10);
const IRON_FILE: ItemRef = ItemRef::simple("iron file", 10);
const IRON_TONGS: ItemRef = ItemRef::simple("iron tongs", 10);
// ---- L ----
const LADDER: ItemRef = ItemRef::simple("10 ft. ladder", 10);
const LENS: ItemRef = ItemRef::simple("lens", 100);
const LOADED_DICE: ItemRef = ItemRef::simple("loaded dice (full set)", 5);
const LOCKPICK: ItemRef = ItemRef::simple("lockpicks", 100);
// ---- M ----
const MANACLES: ItemRef = ItemRef::simple("iron manacles", 10);
const MARBLES: ItemRef = ItemRef::simple("marbles (bag)", 10);
const MIRROR: ItemRef = ItemRef::simple("mirror", 200);
// ---- N ----
const NAILS: ItemRef = ItemRef::simple("nails (bag)", 5);
const NEEDLE: ItemRef = ItemRef::simple("needle", 10);
// ---- P ----
const PADLOCK_AND_KEY: ItemRef = ItemRef::simple("padlock and key", 20);
const PERFUME: ItemRef = ItemRef::simple("perfume", 50);
const PITON: ItemRef = ItemRef::simple("piton (bag)", 15);
const POLE: ItemRef = ItemRef::simple("pole (10 ft)", 5);
// ---- Q ----
const QUILL_AND_INK: ItemRef = ItemRef::simple("quill and ink", 5);
// ---- R ----
const ROPE: ItemRef = ItemRef::simple("50 ft. rope", 10);
// ---- S ----
const SACK: ItemRef = ItemRef::simple("sack", 1);
const SAW: ItemRef = ItemRef::simple("saw", 10);
const SHOVEL: ItemRef = ItemRef::simple("shovel", 10);
const SPONGE: ItemRef = ItemRef::simple("large sponge", 5);
// ---- T ----
const TAR: ItemRef = ItemRef::simple("tar (pot)", 10);
const TENT_PERSONAL: ItemRef = ItemRef::simple("tent (personal)", 50);
const TENT_3_MAN: ItemRef = ItemRef::simple("tent (3 man)", 100);
pub const TORCH: ItemRef = ItemRef::new("torch", 1, &[Prop::Buff(TORCH_BUFF)]);
const TORCH_BUFF: BuffRef = BuffRef::new(
    "torch light",
    &[
        BuffProp::Score(Ability::QuickAccess, -1),
        BuffProp::Effect("a 30 ft circle around the torch is well lit"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const TWINE: ItemRef = ItemRef::simple("twine (300 ft)", 5);
// ---- W ----
const WATERSKIN: ItemRef = ItemRef::simple("waterskin", 5);
const WHISTLE: ItemRef = ItemRef::simple("whistle", 5);

#[rustfmt::skip]
pub const ITEMS: [&ItemRef; 49] = [
    &BEAR_TRAP, &BEDROLL, &BELL, &BELLOWS, &BLOCK_AND_TACKLE, &BOTTLE, &BUCKET, 
    &CARDS, &CHALK, &CHISEL, &CAULDRON, &CROWBAR, 
    &FACE_PAINT, &FISHING_NET, &FISHING_ROD_AND_TACKLE, 
    &GLUE, &GRAPPLING_HOOK,
    &HAND_DRILL, &HAMMER, &HORN,
    &IRON_CHAIN, &IRON_FILE, &IRON_TONGS, 
    &LADDER, &LENS, &LOADED_DICE, &LOCKPICK,
    &MANACLES, &MARBLES, &MIRROR,
    &NAILS, &NEEDLE,
    &PADLOCK_AND_KEY, &PERFUME, &PITON, &POLE, 
    &QUILL_AND_INK,
    &ROPE, 
    &SACK, &SAW, &SHOVEL, &SPONGE,
    &TAR, &TENT_3_MAN, &TENT_PERSONAL, &TORCH, &TWINE,
    &WATERSKIN, &WHISTLE,
];
pub const BUFFS: [&BuffRef; 1] = [&TORCH_BUFF];

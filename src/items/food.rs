use super::ItemRef;

const ALE: ItemRef = ItemRef::simple("ale", 1);
const BREAD_AND_CHEESE: ItemRef = ItemRef::simple("bread and cheese", 6);
const HARD_TACK: ItemRef = ItemRef::simple("hard tack", 5);
const JERKY: ItemRef = ItemRef::simple("jerky", 10);
const WINE: ItemRef = ItemRef::simple("wine", 1);

pub const ITEMS: [&ItemRef; 5] = [&ALE, &BREAD_AND_CHEESE, &HARD_TACK, &JERKY, &WINE];

use super::PC;

mod buffs;
mod class_exp;
mod inventory;
mod misc;

pub fn updater() {
    // The order of these effect calls matters a great deal.
    // If they are incorrectly ordered, certain fn calls won't be made properly.
    class_exp::on_exp();
    misc::collate_abi_scores();
    buffs::on_change();
    buffs::remove_expired();
    inventory::reduce_by_fatigue();
    inventory::on_item_change();
    inventory::encumberance();
}

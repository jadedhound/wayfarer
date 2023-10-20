use std::cmp;

use leptos::*;

use super::buff_view;
use crate::buffs::{Buff, BuffProp};
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::delete_confirm::DeleteModal;

pub(super) fn list() -> impl IntoView {
    let pc = PC::expect();
    // Set what happens when a buff is confirmed for deletion.
    DeleteModal::set_effect(move |id| pc.update_discard(|pc| pc.buffs.remove(id)));
    let buff_list = move || {
        pc.with(|pc| {
            let mut buffs: Vec<_> = pc.buffs.iter().collect();
            buffs.sort_unstable_by(|(_, a), (_, b)| prop_order(a).cmp(&prop_order(b)));
            buffs.into_iter().map(buff_view::view).collect_view()
        })
    };
    let no_buffs = PC::slice(|pc| pc.buffs.is_empty());
    view! {
        <div
            class= "flex flex-col gap-y-1 shaded-table"
            disabled=no_buffs
        >
            { buff_list }
        </div>
    }
}

/// Ordered by what the player should pay most attention to.
fn prop_order(buff: &&Buff) -> usize {
    // Start in the middle as default since it will be overridden.
    let mut cat = 5;
    // Start at the low since categories will add to this.
    let mut sub = 0;

    // Lower is more important and higher numbers are less so.
    buff.props.iter().for_each(|prop| match prop {
        // Mutually exclusive categories.
        BuffProp::Debuff => cat = 0,
        BuffProp::Class => cat = 9,
        // Subcategories that modify the place within categories.
        // Highest importance/doesn't lower importance.
        BuffProp::Effect(_) => (),
        BuffProp::Duration(_) => (),
        BuffProp::StatOverride(_, _) => (),
        BuffProp::Expiry(_) => (),
        // Effects importance.
        BuffProp::Count(_) => sub += 1,
        BuffProp::Rally => sub += 2,
        BuffProp::Rechargable => sub += 3,
        BuffProp::Rest => sub += 3,
    });
    cat * 10 + cmp::min(sub, 9)
}

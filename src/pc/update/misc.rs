use leptos::logging::log;
use leptos::*;
use leptos_use::use_debounce_fn;

use crate::pc::session::Session;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

/// Collates the 3 different `AbiScores` in `Session` to get the final values.
pub fn collate_abi_scores() {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let scores = Session::slice(|sesh| {
        (
            sesh.isolated_scores,
            sesh.inv_scores,
            sesh.buff_scores,
            sesh.override_scores.clone(),
        )
    });
    let debounced = use_debounce_fn(
        move || {
            log!("> Collating AbiScore changes");
            let mut base = pc.with_untracked(|pc| pc.abi_scores);
            let (isolated, inventory, buff, overrides) = scores.get();
            let add_scores = isolated
                .iter_enum()
                .chain(buff.iter_enum())
                .chain(inventory.iter_enum());
            for (abi, score) in add_scores {
                *base.get_mut(abi) += score
            }
            for (abi, score) in overrides {
                *base.get_mut(abi) = score
            }
            sesh.update(|sesh| sesh.abi_scores = base);
        },
        100.0,
    );

    create_effect(move |_| {
        let _ = scores.get();
        debounced()
    });
}

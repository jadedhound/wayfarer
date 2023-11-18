use leptos::logging::log;
use leptos::*;

use crate::pc::session::Session;
use crate::pc::{Ability, PC};
use crate::utils::rw_utils::RwUtils;

pub fn on_exp() {
    let sesh = Session::expect();
    let exp = PC::slice(|pc| pc.class.1);
    let level = Session::slice(|sesh| sesh.level);

    create_render_effect(move |_| {
        log!("> ClassExp changed (RENDER)");
        let exp = exp.get();
        sesh.update(|sesh| sesh.level = exp.level());
    });
    create_effect(move |_| {
        log!("> ClassLevel changed");
        let _ = level.get();
        health_increase();
    });
}

fn health_increase() {
    log!("    | Calculating health and guard increase");
    let guard_bonus = PC::expect().with_untracked(|pc| pc.class.0.guard_bonus);
    Session::expect().update(|sesh| {
        let level = sesh.level.get() as i32;
        let health = level - 1;
        let guard = (level - 1) * guard_bonus;
        *sesh.isolated_scores.get_mut(Ability::Health) = health;
        *sesh.isolated_scores.get_mut(Ability::Guard) = guard;
    })
}

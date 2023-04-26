use std::collections::HashMap;

use leptos::*;
use leptos_router::use_params_map;
use serde::Deserialize;

use crate::{
    errors::*,
    utils::{fetch, ToView},
};

#[derive(Deserialize, Clone)]
enum Rank {
    Novice,
    Expert,
    Master,
}

#[derive(Deserialize, Clone)]
struct Spell {
    rank: Rank,
    stip: Option<String>,
    effect: String,
}

#[derive(Deserialize, Clone)]
struct SpellList {
    arcane: HashMap<String, Spell>,
    divine: HashMap<String, Spell>,
}

#[derive(Clone)]
struct ShowArcane(bool);

#[component]
pub fn Spellbook(cx: Scope) -> impl IntoView {
    let school = move || use_params_map(cx).get().get("school").cloned();
    let spellbook = create_local_resource(
        cx,
        || (),
        |_| async move { fetch::<SpellList>("spells.json".into()).await },
    );

    // Setting which school is shown.
    let (get_sch, set_sch) = create_signal(cx, ShowArcane(true));
    let show_arcane = move || get_sch.get().0;
    provide_context(cx, set_sch);

    view! {
        cx,
        {move || spellbook.read(cx).blank_or(cx, |data| {
            match data {
                Ok(data) => {
                    provide_context(cx, data);
                    view!{ cx,
                        <h1> "Spellbook" </h1>
                    }.into_view(cx)
                },
                Err(e) => {
                    let reason = e.to_string();
                    view!{ cx, <FatalError code= "400" reason=&reason /> }.into_view(cx)
                }
            }
        })}
    }
}

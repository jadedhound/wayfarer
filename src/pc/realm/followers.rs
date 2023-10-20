use leptos::*;
use strum::IntoEnumIterator;

use super::{Follower, FollowerStat, FollowerStatArray};
use crate::icons;
use crate::pc::PC;
use crate::rand::Rand;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::Turns;
use crate::utils::RwSignalEnhance;
use crate::views::delete_confirm::DeleteModal;

pub fn followers() -> impl IntoView {
    let pc = PC::expect();
    DeleteModal::set_effect(move |id| pc.update_discard(|pc| pc.followers.remove(id)));
    let no_followers = move || pc.with(|pc| pc.followers.is_empty());
    let follower_list = move || pc.with(|pc| pc.followers.iter().map(follower_view).collect_view());

    view! {
        <div class= "shaded-table" hidden=no_followers>
            { follower_list }
        </div>
        { random_follower_button }
    }
}

fn random_follower_button() -> impl IntoView {
    let pc = PC::expect();
    let max_followers = PC::slice(|pc| pc.followers.len() > 2);
    let cooldown = PC::slice(|pc| pc.follower_cooldown.sub(pc.turns));
    let disabled = move || max_followers.get() || cooldown.get().0 > 0;
    let add_follower = move |_| {
        let follower = Rand::with(gen_follower);
        pc.update(|pc| {
            pc.followers.add(follower);
            if !cfg!(debug_assertions) {
                pc.follower_cooldown = pc.turns;
                pc.follower_cooldown.add(Turns::new(3, 0));
            }
        });
    };
    let text = move || {
        if cooldown.get().0 > 0 {
            let days = move || format!("{} DAY COOLDOWN", cooldown.get().in_days());
            view! {
                <div class= "w-6 -translate-y-px" inner_html=icons::CLOCK />
                <div> { days } </div>
            }
            .into_view()
        } else {
            view! {
                <div class= "w-6 -translate-y-px" inner_html=icons::DIE />
                <div> "RANDOM FOLLOWER" </div>
            }
            .into_view()
        }
    };

    view! {
        <button
            class= "btn bg-surface py-2 flex justify-center gap-2"
            on:click=add_follower
            disabled=disabled
        >
            { text }
        </button>
    }
}

fn follower_view((id, follower): (usize, &Follower)) -> impl IntoView {
    view! {
        <div class= "flex gap-2 px-2">
            <button on:click=move |_| DeleteModal::show(id)>
                <div class= "w-5 fill-red-600" inner_html=icons::TRASH />
            </button>
            <button
                class= "p-2 w-full"
            >
                { follower.into_view() }
            </button>
        </div>
    }
}

fn gen_follower(rand: &mut Rand) -> Follower {
    let mut name = rand.pick(&names::NAMES).to_string();
    let mut stats = FollowerStatArray::default();
    let incr_stats = rand_incr_stats(rand);
    // 1/3 chance of the follower having a title and better stats.
    if rand.range(0, 2) == 0 {
        name.push(' ');
        name.push_str(rand.pick(&names::TITLES));
        for (i, stat) in incr_stats.iter().enumerate() {
            *stats.get_mut(*stat) += i as i32 + 1;
        }
    } else {
        *stats.get_mut(incr_stats[0]) += 1;
    }
    Follower {
        name,
        level: 0,
        stats,
    }
}

/// Gives 2 stats from `FolStat`.
/// Used in `gen_follower` to give stat variety.
fn rand_incr_stats(rand: &mut Rand) -> [FollowerStat; 2] {
    let fol_stats: Vec<_> = FollowerStat::iter().collect();
    let first = rand.pick(&fol_stats);
    let second = rand.pick(&fol_stats);
    // Reroll if the chosen stats are the same.
    if first as usize == second as usize {
        rand_incr_stats(rand)
    } else {
        [first, second]
    }
}

#[rustfmt::skip]
mod names {
    pub const NAMES: [&str; 80] = [
        // MALE
        "Zane", "Kato", "Ragnor", "Tharion", "Jace", "Lorn", "Eron", "Varis", "Dorian", "Kira", 
        "Nix", "Zephyr", "Corin", "Razel", "Lysander", "Orion", "Kian", "Draven", "Zarek", "Rian", 
        "Talon", "Kyrus", "Soren", "Zevran", "Lucian", "Rix", "Cale", "Torin", "Darius", "Kairos", 
        "Zalazar", "Remy", "Leon", "Kael", "Xander", "Corvin", "Rafe", "Zane", "Kato", "Ragnor",
        // FEMALE
        "Aria", "Lila", "Elora", "Nyx", "Selene", "Zara", "Lyra", "Rina", "Kira", "Mara", "Lira", 
        "Nia", "Sela", "Zora", "Eira", "Ryla", "Kyla", "Tara", "Lora", "Nara", "Zira", "Eila", 
        "Rora", "Kora", "Mira", "Lena", "Zena", "Eira", "Rina", "Kina", "Sara", "Zola", "Eila", 
        "Rila", "Kala", "Mala", "Lina", "Zana", "Eina", "Rana"
    ];
    pub const TITLES: [&str; 36] = [
        "the Swift", "the Wise", "the Fierce", "the Silentflame", "the Shadow", "the Brave", "the Storm", 
        "the Cunning", "the Noble", "the Flame", "the Silent", "the Wind", "the Bold", "the Mad", 
        "the Fair", "the Star", "the Bright", "the Dark", "the Iron", "the Hunter", "the Claw", 
        "the Frost", "the Wise", "the Rogue", "the Light", "the Trickster", "the Wild", "the Strong", 
        "the King", "the Timeless", "the Dragon", "the Lucky", "the Lion", "the Blade", 
        "the Raven", "the Rebel",
    ];
}

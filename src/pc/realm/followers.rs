use leptos::*;
use strum::IntoEnumIterator;

use super::{FolStat, FolStatArray, Follower};
use crate::icons;
use crate::pc::PC;
use crate::rand::Rand;
use crate::utils::RwProvided;
use crate::views::delete_btn::{delete_btn, delete_btn_show};

pub fn followers() -> impl IntoView {
    let follower_list = move || {
        PC::with(|pc| {
            if pc.followers.is_empty() {
                view! {
                    <div class= "italic text-center">
                        "Followers are an invaluable resource. You can have a maximum of 3
                        and can roll for 1 per day while in a safe area."
                    </div>
                }
            } else {
                let arr = pc.followers.iter().map(follower_view).collect_view();
                view! {
                    <div class= "flex flex-col gap-y-2 shaded-table">
                        { arr }
                    </div>
                }
            }
        })
    };

    view! {
        <div class= "flex flex-col gap-4">
            { follower_list }
            { random_follower_btn }
        </div>
    }
}

fn random_follower_btn() -> impl IntoView {
    let fol_result = RwSignal::new(None);
    let too_many_fol = move || PC::with(|pc| pc.followers.len() > 2);
    let gen_fol_result = move |_| {
        let fol = Rand::with(gen_follower);
        fol_result.set(Some(fol))
    };
    let rand_btn = move || {
        view! {
            <button
                class= "btn bg-surface py-2 flex justify-center gap-2"
                on:click=gen_fol_result
                disabled=too_many_fol
            >
                <div class= "w-6 -translate-y-px" inner_html=icons::DIE />
                <div> "RANDOM FOLLOWER" </div>
            </button>
        }
    };
    let reset_fol_result = move |_| fol_result.set(None);
    let add_follower = move |fol| {
        PC::update(|pc| pc.followers.add(fol));
        fol_result.set(None)
    };
    let confirm_choice = move |fol: Follower| {
        let fol_view = fol.into_view();
        view! {
            <div class= "flex gap-1">
                <button
                    class= "btn-no-font bg-surface w-12 grow p-2"
                    on:click=move |_| add_follower(fol.clone())
                >
                    { fol_view }
                </button>
                <button
                    class= "btn bg-red-800 px-2"
                    on:click=reset_fol_result
                >
                    <div class= "w-4" inner_html=icons::CROSS />
                </button>
            </div>
        }
    };

    move || {
        if let Some(fol) = fol_result.get() {
            confirm_choice(fol).into_view()
        } else {
            rand_btn().into_view()
        }
    }
}

fn follower_view((id, follower): (usize, &Follower)) -> impl IntoView {
    let delete = move || PC::update(|pc| pc.followers.remove(id));

    view! {
        <div class= "relative">
            <div
                class= "p-2"
                on:contextmenu=delete_btn_show('f', id)
            >
                { follower.into_view() }
            </div>
            { delete_btn('f', id, delete)}
        </div>
    }
}

fn gen_follower(rand: &mut Rand) -> Follower {
    let mut name = rand.pick(&names::NAMES).to_string();
    let mut stats = FolStatArray::default();
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
fn rand_incr_stats(rand: &mut Rand) -> [FolStat; 2] {
    let fol_stats: Vec<_> = FolStat::iter().collect();
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

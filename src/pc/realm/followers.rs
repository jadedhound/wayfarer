use leptos::*;

use super::{Experience, Follower};
use crate::icons;
use crate::pc::PC;
use crate::utils::{expect_rw, some_if, RwProvided};

pub fn followers() -> impl IntoView {
    let follower_view = move || {
        PC::with(|pc| {
            let has_followers = !pc.followers.is_empty();
            some_if(has_followers).map(|_| {
                let arr = pc
                    .followers
                    .iter()
                    .map(|(id, x)| follower_view(id, x))
                    .collect_view();
                view! {
                    <div class= "flex flex-col gap-y-2 shaded-table">
                        { arr }
                    </div>
                }
            })
        })
    };

    view! {

        <div class= "flex flex-col gap-4">
            { follower_view }
            { add_follower_input }
        </div>
    }
}

fn add_follower_input() -> impl IntoView {
    let pc = expect_rw::<PC>();
    let new_name = create_rw_signal(String::new());
    let add_follower = move || {
        let name = new_name.get();
        new_name.set(String::new());
        pc.update(|pc| {
            pc.followers.add(Follower {
                name,
                exp: Experience::Novice,
            })
        })
    };

    move || {
        some_if(pc.with(|pc| pc.followers.len() < 3)).map(|_| {
            view! {
                <div class= "flex gap-1">
                    <input
                        class= "w-12 grow input"
                        on:input=move |ev| new_name.set(event_target_value(&ev))
                        prop:value=move || new_name.get()
                    />
                    <button
                        class= "btn bg-green-800 self-center text-xl w-10"
                        on:click=move |_| add_follower()
                        disabled=move || new_name.get().is_empty()
                        inner_html=icons::PLUS
                    />
                </div>
            }
        })
    }
}

fn follower_view(id: usize, follower: &Follower) -> impl IntoView {
    let pc = expect_rw::<PC>();
    let delete = move || {
        pc.update(|pc| {
            pc.followers.remove(id);
        })
    };
    let upgrade = move || {
        pc.update(|pc| {
            let follower = pc.followers.get_mut(id).unwrap();
            follower.exp = Experience::from_repr(follower.exp as usize + 1).unwrap_or_default();
        })
    };

    view! {
        <div class= "flex">
            <div class= "p-2 flex flex-col justify-center w-full uppercase">
                <div> { follower.name.clone() } </div>
                <div class= "text-sm italic"> { format!("Inventory +{}", follower.inv_incr()) } </div>
            </div>
            <button
                class= "w-24 text-sky-500"
                on:click=move |_| upgrade()
            >
                { follower.exp.to_string() }
            </button>
            <button
                class= "self-center px-2 h-full text-red-800 text-center text-2xl"
                on:click=move |_| delete()
            >
                <div class= "w-4 fill-red-500" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

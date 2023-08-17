use leptos::*;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::FromRepr;

use crate::pc::pc_stat::PCStat;
use crate::svg;
use crate::utils::RwProvided;
use crate::utils::expect_rw;
use crate::utils::some_if;

use super::PC;

#[derive(Serialize, Deserialize, Clone, Copy, Display, FromRepr, Default)]
pub enum Experience {
    #[default]
    Novice,
    Journeyman,
    Expert,
    Master,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Follower {
    pub name: String,
    pub exp: Experience,
}

impl Follower {
    pub fn inv_incr(&self) -> usize {
        2 * (self.exp as usize + 1)
    }
}

pub fn followers() -> impl IntoView {
    let follower_view = move || {
        PC::with( |pc| {
            let has_followers = !pc.followers.is_empty();
            some_if(has_followers).map(|_| {
                let arr = pc.followers
                    .iter()
                    .map(|(id, x)| follower_view( id, x))
                    .collect_view();
                view!{ 
                    <div class= "flex flex-col gap-y-2 shaded-table">
                        { arr }
                    </div>
                }
            })
        })  
    };

    view! {
        
        <div class= "flex flex-col px-2 gap-4">
            <h5 class= "text-center"> "ROSTER" </h5>
            { follower_view }
            { add_follower_input() }
        </div>
    }
}

fn add_follower_input() -> impl IntoView {
    let pc = expect_rw::<PC>();
    let new_name = create_rw_signal( String::new());
    let add_follower = move || {
        let name = new_name.get();
        new_name.set(String::new());
        pc.update(|pc| {
            let follower = Follower {
                name,
                exp: Experience::Novice,
            };
            *pc.base_stats.get_mut(PCStat::Inventory) += follower.inv_incr() as i32;            
            pc.followers.add(follower);

        })
    };

    move || {
        some_if(pc.with(|pc| pc.followers.len() < 3)).map(|_| {
             view!{ 
                <div class= "flex mb-6">
                    <input
                        class= "w-12 grow rounded-l bg-inherit outline-none border-y-2 border-l-2 border-pink-800 px-2"
                        on:input=move |ev| new_name.set(event_target_value(&ev))
                        prop:value=move || new_name.get()
                    />
                    <button
                        class= "btn-rounded-r bg-pink-800 disabled:border-pink-800 flex-centered w-12"
                        on:click=move |_| add_follower()
                        disabled=move || new_name.get().is_empty()
                    >
                        <div class= "svg w-6" inner_html=svg::PLUS />
                    </button>
                </div>
            }           
        })
    }
}

fn follower_view( id: usize, follower: &Follower) -> impl IntoView {
    let pc = expect_rw::<PC>();
    let delete = move || {
        pc.update(|pc| {
            if let Some(x) = pc.followers.remove(id) {
                *pc.base_stats.get_mut(PCStat::Inventory) -= x.inv_incr() as i32;
            }
        })
    };
    let upgrade = move || {
        pc.update(|pc| {
            let inv_stat = pc.base_stats.get_mut(PCStat::Inventory);
            let follower = pc.followers.get_mut(id).unwrap();
            *inv_stat -= follower.inv_incr() as i32;
            follower.exp = Experience::from_repr(follower.exp as usize + 1).unwrap_or_default();
            *inv_stat += follower.inv_incr() as i32;
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
                class= "px-2"
                on:click=move |_| delete()
            >
                <div class= "stroke-red-800 w-4" inner_html=svg::CROSS />
            </button>
        </div>
    }
}

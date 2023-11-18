use leptos::*;
use web_sys::Event;

use super::State;
use crate::icons;
use crate::items::ItemRef;
use crate::pc::realm::shop::shop_item_view;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

pub fn shop_view() -> impl IntoView {
    let state = State::expect();
    let shop = state.with_untracked(|state| state.shop);
    let to_item = move |item_ref: &'static ItemRef| {
        let weight = item_ref.is_bulky() as usize + 1;
        let add_to_cart = move || {
            state.update(|state| {
                state.weight += weight;
                state.price += item_ref.price();
                state.cart.add(item_ref);
            })
        };
        shop_item_view(item_ref, add_to_cart)
    };
    let shop_items = move || {
        state.with(|state| {
            state
                .search_results
                .as_ref()
                .unwrap_or(&state.shop.items().to_vec())
                .iter()
                .map(|item| to_item(item))
                .collect_view()
        })
    };

    view! {
        <h3 class= "text-center"> { shop.to_string() } </h3>
        <div class= "italic text-center"> { shop.desc() } </div>
        { search_filter }
        <div class= "flex flex-col shaded-table empty:hidden">
            { shop_items }
        </div>
    }
}

fn search_filter() -> impl IntoView {
    let state = State::expect();
    let shop = state.with_untracked(|state| state.shop);
    let query = RwSignal::new(String::new());
    let search_query = move |ev: Event| {
        let usr_search = event_target_value(&ev).to_lowercase();
        query.set(usr_search.clone());
        let results = (!usr_search.is_empty()).then(move || {
            shop.items()
                .iter()
                .filter(|item| item.name.contains(&usr_search))
                .copied()
                .collect()
        });
        state.update(|state| state.search_results = results);
    };
    let clear_query = move |_| {
        query.reset();
        state.update(|state| state.search_results = None);
    };

    view! {
        <div class= "relative">
            <div class= "absolute inset-y-0 left-2 flex-center" >
                <div class= "w-6 stroke-sky-500" inner_html=icons::MAGNIFYING_GLASS />
            </div>
            <input
                class= "input text-center w-full !px-10"
                placeholder= "Item search..."
                on:input=search_query
                prop:value=query
            />
            <button
                class= "absolute inset-y-0 right-2 flex-center fill-sky-500 disabled:fill-zinc-500"
                on:click=clear_query
                disabled=move || query.with(|query|query.is_empty())
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

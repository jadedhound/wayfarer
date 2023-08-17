use std::time::Duration;

use leptos::*;

use crate::items::item_spec::ItemSpec;
use crate::items::search::{search, SearchResult};
use crate::items::{Item, ItemQuality};
use crate::pc::inventory::funds::fund_formatted_input;
use crate::pc::PC;
use crate::svg;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::modal::{ModalCentered, ModalState};

#[derive(Default)]
struct SearchState {
    query: String,
    is_active: bool,
}

struct FoundItem(Option<Item>);

pub(super) fn search_view() -> impl IntoView {
    let state = create_rw_signal(SearchState::default());
    provide_context(state);
    let found = create_rw_signal(FoundItem(None));
    provide_context(found);
    let custom_item = create_rw_signal(CustomItem::default());
    provide_context(custom_item);

    let item_or_textbox = move || {
        if let Some(item) = found.with(|x| x.0.clone()) {
            item_view(item).into_view()
        } else {
            textbox().into_view()
        }
    };
    let hide_results = move || state.with(|x| !x.is_active);
    let open_custom_item = move || {
        custom_item.update(|x| *x = CustomItem::default());
        ModalState::open(0)
    };

    view! {
        <div class= "flex flex-col gap-2">
            { item_or_textbox }
            <div hidden=hide_results>
                <div class= "flex flex-col gap-1">
                    <div class= "flex flex-col shaded-table">
                        { results_view() }
                    </div>
                    <button
                        class= "font-sans rounded border-2 border-zinc-800 text-green-600 py-2"
                        on:click=move |_| open_custom_item()
                    >
                        "CUSTOM ITEM"
                    </button>
                </div>
            </div>
        </div>
        { custom_item_modal() }
    }
}

fn item_view(item: Item) -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let found = expect_rw::<FoundItem>();
    let item_v = item.into_view();
    let add_item = move || {
        PC::update(|pc| pc.inventory.add(item.clone()));
        found.update(|x| x.0 = None);
        state.update(|x| x.query = String::new());
    };
    let del_item = move || {
        found.update(|x| x.0 = None);
        state.update(|x| x.query = String::new())
    };

    view! {
        <div class= "flex gap-1">
            <button
                class= "rounded bg-zinc-800 py-1 w-12 grow"
                on:click=move |_| add_item()
            >
                { item_v }
            </button>
            <button
                class= "rounded flex-centered w-10 bg-red-800"
                on:click=move |_| del_item()
            >
                <div class= "w-6 svg" inner_html=svg::CROSS />
            </button>
        </div>
    }
}

fn textbox() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    // Focus loss needs to be staggered so that search results can be
    // clicked.
    let delayed_loss = move || {
        spawn_local(async move {
            gloo::timers::future::sleep(Duration::from_millis(1)).await;
            // Delay means that state might be disposed of, e.g. quickly
            // navigating after clicking on input.
            if let Some(x) = use_context::<RwSignal<SearchState>>() {
                x.update(|state| state.is_active = false)
            }
        })
    };

    view! {
        <input
            class= "rounded h-10 bg-zinc-800 outline-none px-1 text-center"
            on:input=move |ev| state.update(|x| x.query = event_target_value(&ev))
            prop:value=move || state.with(|x| x.query.clone())
            on:focus=move |_| state.update(|x| x.is_active = true)
            on:blur=move |_| delayed_loss()
        />
    }
}

fn results_view() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let found = expect_rw::<FoundItem>();
    let query =
        create_memo(move |_| state.with(|x| some_if(!x.query.is_empty()).map(|_| x.query.clone())));
    let empty = move || {
        view! {
            <div class= "text-center py-2"> "Search for items or loot tables..." </div>
        }
    };
    let add_item = move |res: &SearchResult| {
        found.update(|x| x.0 = Some(res.to_item()));
        state.update(|x| x.is_active = false);
    };
    let results_v = move |x: &str| {
        let arr: Vec<_> = search(x).take(3).collect();
        if arr.is_empty() {
            view! {
                <div class= "text-center py-2">
                    "Unable to find any items..."
                </div>
            }
            .into_view()
        } else {
            arr.into_iter()
                .map(|res| {
                    let name = res.to_string();
                    view! {
                        <button
                            class= "text-center py-2"
                            on:click=move |_| add_item(&res)
                        >
                            { name }
                        </button>
                    }
                })
                .collect_view()
        }
    };

    move || match query.get() {
        Some(x) => results_v(&x).into_view(),
        None => empty.into_view(),
    }
}

struct CustomItem(Item);
impl Default for CustomItem {
    fn default() -> Self {
        Self(Item {
            name: String::new(),
            spec: ItemSpec::Simple,
            quality: ItemQuality::Common,
            is_bulky: false,
            // Note: this price isn't changed until the item is added.
            price: 0,
            stacks: None,
        })
    }
}

fn custom_item_modal() -> impl IntoView {
    let item = expect_rw::<CustomItem>();
    let name = move || item.with(|x| x.0.name.clone());
    let name_set = move |y| item.update(|x| x.0.name = y);
    let quality = move || item.with(|x| x.0.quality.to_string());
    let quality_set = move || {
        item.update(|x| {
            let curr = x.0.quality as usize;
            x.0.quality = ItemQuality::from_repr(curr + 1).unwrap_or_default()
        })
    };
    let is_bulky = move || item.with(|x| x.0.is_bulky);
    let is_bulky_set = move || item.update(|x| x.0.is_bulky = !x.0.is_bulky);
    let price = create_rw_signal(0);
    let stacks = move || {
        item.with(|x| {
            if let Some((_, max)) = x.0.stacks {
                format!("Up to {}", max)
            } else {
                "None".into()
            }
        })
    };
    let stacks_set = move || {
        item.update(|x| {
            if let Some((_, max)) = x.0.stacks.as_mut() {
                if *max >= 5 {
                    x.0.stacks = None
                } else {
                    *max += 1
                }
            } else {
                x.0.stacks = Some((1, 1))
            }
        })
    };
    let not_valid_item = move || item.with(|x| x.0.name.is_empty());
    let add_item = move || {
        PC::update(|pc| {
            let mut item = item.with(|x| x.0.clone());
            item.price = price.get();
            price.set(0);
            pc.inventory.add(item)
        });
        ModalState::dismiss()
    };

    view! {
        <ModalCentered title=|| "CUSTOM ITEM" id=0>
            <div class= "grid grid-cols-3 gap-2">
                <div> "Name" </div>
                <input
                    class= "col-span-2 rounded outline-none border-2 border-sky-800 py-1 bg-inherit"
                    on:input=move |ev| name_set(event_target_value(&ev))
                    prop:value=name
                />
                <div> "Quality" </div>
                <button
                    class= "col-span-2 py-1"
                    on:click=move |_| quality_set()
                >
                    { quality }
                </button>
                <div> "Is bulky?" </div>
                <input
                    class= "col-span-2"
                    type= "checkbox"
                    on:click=move |_| is_bulky_set()
                    prop:checked=is_bulky
                />
                <div> "Price" </div>
                <div class= "col-span-2">
                    { fund_formatted_input( price) }
                </div>
                <div> "Stacks" </div>
                <button
                    class= "col-span-2"
                    on:click=move |_| stacks_set()
                >
                    { stacks }
                </button>
            </div>
            <button
                class= "bg-green-700 rounded disabled:bg-inherit disabled:border-2 disabled:border-zinc-700"
                on:click=move |_| add_item()
                disabled=not_valid_item
            >
                "CREATE"
            </button>
        </ModalCentered>
    }
}

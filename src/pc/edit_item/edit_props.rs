use std::cmp;

use leptos::*;

use super::State;
use crate::icons;
use crate::items::ItemProp as Prop;
use crate::utils::counter::Counter;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

pub fn edit_props() -> impl IntoView {
    let state = State::expect();
    let prop_ids = move || {
        state.with(|state| {
            state
                .item
                .props
                .iter()
                .enumerate()
                .map(|(i, prop)| (i, prop.index()))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class= "shaded-table flex flex-col gap-2 empty:hidden">
            <For
                each=prop_ids
                key=|(_, index)| *index
                children=delegate
            />
        </div>
    }
}

fn delegate((i, _): (usize, usize)) -> impl IntoView {
    let state = State::expect();
    match state.with(|state| state.item.props.get(i).cloned().unwrap()) {
        Prop::Bulky(_) => bulky(i),
        Prop::Concentration => simple(i),
        Prop::Count(_) => counter(i),
        Prop::Damage(_) => damage(i),
        Prop::Effect(_) => effect(i),
        Prop::Range(_) => range(i),
        Prop::Resist => simple(i),
        Prop::Usable(_) => usable(i),
        Prop::Passive => simple(i),
        _ => None,
    }
    .into_view()
}

fn prop_slice<T, I, GM, G, S>(
    i: usize,
    getter: G,
    get_mut: GM,
    setter: S,
) -> (Signal<T>, SignalSetter<T>)
where
    T: Default + PartialEq,
    G: Fn(&Prop) -> Option<T> + Copy + 'static,
    GM: Fn(&mut Prop) -> Option<&mut I> + Copy + 'static,
    S: Fn(&mut I, T) + Copy + 'static,
{
    State::rw_slice(
        move |state| state.item.props.get(i).and_then(getter).unwrap_or_default(),
        move |state: &mut State, value: T| {
            let curr = state.item.props.get_mut(i).and_then(get_mut);
            if let Some(curr) = curr {
                setter(curr, value)
            }
        },
    )
}

fn counter(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Count(x) => Some(x.curr),
            _ => None,
        },
        |prop| match prop {
            Prop::Count(x) => Some(x),
            _ => None,
        },
        |count, value| *count = Counter::new(cmp::max(value, 1)),
    );
    row_number(i, value, set_value, 2, 15)
}

fn bulky(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Bulky(x) => Some(*x),
            _ => None,
        },
        |prop| match prop {
            Prop::Bulky(x) => Some(x),
            _ => None,
        },
        |curr, value| *curr = value,
    );
    row_number(i, value, set_value, 2, 8)
}

fn damage(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Damage(x) => Some(*x),
            _ => None,
        },
        |prop| match prop {
            Prop::Damage(x) => Some(x),
            _ => None,
        },
        |curr, value| *curr = value,
    );
    row_number(i, value, set_value, 0, 9)
}

fn effect(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Effect(x) => Some(x.clone()),
            _ => None,
        },
        |prop| match prop {
            Prop::Effect(x) => Some(x),
            _ => None,
        },
        |curr, value| *curr = value,
    );
    row_text(i, value, set_value, "An effect that is inherent to the item and requires no additional resources, usually passive in nature.")
}

fn range(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Range(x) => Some(usize::try_from(*x).unwrap_or_default()),
            _ => None,
        },
        |prop| match prop {
            Prop::Range(x) => Some(x),
            _ => None,
        },
        |curr, value| *curr = u32::try_from(value).unwrap_or_default(),
    );
    row_large_number(i, value, set_value)
}

fn usable(i: usize) -> Option<View> {
    let (value, set_value) = prop_slice(
        i,
        |prop| match prop {
            Prop::Usable(x) => Some(x.clone()),
            _ => None,
        },
        |prop| match prop {
            Prop::Usable(x) => Some(x),
            _ => None,
        },
        |curr, value| *curr = value,
    );
    row_text(i, value, set_value, "An effect that requires the use of the item (decrementing the count or destroying the item) to activate.")
}

fn row_text(
    i: usize,
    value: Signal<String>,
    set_value: SignalSetter<String>,
    placeholder: &'static str,
) -> Option<View> {
    Some(
        view! {
            <PropRow i>
                <textarea
                    class= "input w-12 grow h-24 !py-0"
                    placeholder=placeholder
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    prop:value=value
                />
            </PropRow>
        }
        .into_view(),
    )
}

fn row_number(
    i: usize,
    value: Signal<usize>,
    set_value: SignalSetter<usize>,
    min: usize,
    max: usize,
) -> Option<View> {
    let lowest = move || value.get() <= min;
    let highest = move || value.get() >= max;
    let incr = move |_| set_value.set(value.get() + 1);
    let decr = move |_| set_value.set(value.get() - 1);

    Some(
        view! {
            <PropRow i>
                <div class= "psuedo w-1 grow" />
                <button
                    class= "disabled:fill-zinc-500"
                    disabled=lowest
                    on:click=decr
                >
                    <div class= "w-5 rotate-180" inner_html=icons::RIGHT_CHEV />
                </button>
                { value }
                <button
                    class= "disabled:fill-zinc-500"
                    disabled=highest
                    on:click=incr
                >
                    <div class= "w-5" inner_html=icons::RIGHT_CHEV />
                </button>
                <div class= "psuedo w-1 grow" />
            </PropRow>
        }
        .into_view(),
    )
}

fn row_large_number(
    i: usize,
    value: Signal<usize>,
    set_value: SignalSetter<usize>,
) -> Option<View> {
    let parse_input =
        move |ev| set_value.set(event_target_value(&ev).parse::<usize>().unwrap_or_default());
    Some(
        view! {
            <PropRow i>
                <input
                    class= "input w-12 grow text-center"
                    type= "number"
                    on:input=parse_input
                    prop:value=value
                />
            </PropRow>
        }
        .into_view(),
    )
}

/// A prop which doesn't have any editable properties.
fn simple(i: usize) -> Option<View> {
    Some(
        view! {
            <PropRow i>
                <div class= "psuedo w-1 grow" />
            </PropRow>
        }
        .into_view(),
    )
}

#[component]
fn PropRow(i: usize, children: Children) -> impl IntoView {
    let state = State::expect();
    let name = state.with_untracked(move |state| {
        state
            .item
            .props
            .get(i)
            .map(|prop| prop.to_string())
            .unwrap_or_default()
    });
    let find_and_delete = move |_| state.update_discard(|state| state.item.props.remove(i));

    view! {
        <div class= "flex items-center p-2 gap-4">
            <div class= "font-tight"> { &name } </div>
            { children() }
            <button
                class= "fill-red-500"
                on:click=find_and_delete
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

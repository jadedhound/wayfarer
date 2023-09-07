use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::{expect_rw, RwProvided};

struct ProfState {
    editing: bool,
    msg: String,
}

pub(super) fn prof_view() -> impl IntoView {
    let state = PC::with(|pc| {
        create_rw_signal(ProfState {
            editing: false,
            msg: pc.prof.clone(),
        })
    });
    provide_context(state);
    let is_editing = create_read_slice(state, |x| x.editing);
    move || {
        if is_editing.get() {
            input_view().into_view()
        } else {
            display_prof().into_view()
        }
    }
}

fn display_prof() -> impl IntoView {
    let state = expect_rw::<ProfState>();
    let open_input = move || state.update(|x| x.editing = true);
    let msg = move || state.with(|x| format!("You are proficient in {} checks.", x.msg));

    view! {
        <button class= "btn bg-surface flex-center" on:click=move |_| open_input()>
            <div class= "w-5" inner_html=icons::QUILL />
        </button>
        <div class= "col-span-6">
            { msg }
        </div>
    }
}

fn input_view() -> impl IntoView {
    let state = expect_rw::<ProfState>();
    let close_input = move || {
        state.update(|x| x.editing = false);
        PC::update(|pc| pc.prof = state.with(|x| x.msg.clone()));
    };
    let val = move || state.with(|x| x.msg.clone());
    let set_val = move |ev| state.update(|x| x.msg = ev);

    view! {
        <button class= "btn bg-green-800 flex-center" on:click=move |_| close_input()>
            <div class= "w-4" inner_html=icons::CHECKMARK />
        </button>
        <input
            class= "col-span-6 input"
            on:input=move |ev| set_val(event_target_value(&ev))
            prop:value=val
        />
    }
}

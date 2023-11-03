use gloo::file::{Blob, ObjectUrl};
use leptos::html::Input;
use leptos::logging::error;
use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

use super::State;
use crate::utils::rw_utils::RwUtils;
use crate::views::confirm_button::ConfirmButton;
use crate::views::revealer::RevLocation;
use crate::views::toast::Toast;
use crate::{icons, indexeddb};

pub fn database() -> impl IntoView {
    view! {
        { db_size }
        { export_db }
        { import_db }
        { delete_database }
    }
}

async fn update_db_size(state: RwSignal<State>) {
    match indexeddb::size().await {
        Ok(size) => {
            state.try_update(|state| state.db_size = size);
        }
        Err(err) => logging::error!("Error: {err}"),
    };
}

fn db_size() -> impl IntoView {
    let state = State::expect();
    let db_size = move || state.with(|state| state.db_size);
    spawn_local(async move { update_db_size(state).await });

    view! {
        <div class= "font-tight text-center">
            { move || format!("CURRENT SIZE: {} KB", db_size() / 1024) }
        </div>
    }
}

fn export_db() -> impl IntoView {
    fn create_invisi_link(url: &str) -> Result<(), JsValue> {
        let doc = document();
        let link = doc.create_element("a")?;
        link.set_attribute("href", url)?;
        link.set_attribute("download", "wayfarer.bin")?;
        let link = doc.body().unwrap().append_child(&link)?;
        let link: HtmlElement = link.dyn_into()?;
        link.click();
        doc.body().unwrap().remove_child(&link)?;
        Ok(())
    }

    let disabled = State::slice(|state| state.db_size == 0);
    let create_invisi_link = |data: Vec<u8>| {
        let blob = Blob::new(data.as_slice());
        let url = ObjectUrl::from(blob);
        if let Err(err) = create_invisi_link(&url) {
            logging::error!("Unable to export: {err:?}")
        }
    };
    let assemble_data = move |_| {
        spawn_local(async move {
            match indexeddb::export().await {
                Ok(data) => create_invisi_link(data),
                Err(err) => logging::error!("Unable to export: {err}"),
            }
        })
    };

    view! {
        <button
            class= "btn bg-sky-800"
            on:click=assemble_data
            disabled=disabled
        >
            "EXPORT"
        </button>
    }
}

fn import_db() -> impl IntoView {
    async fn get_file(input_ref: NodeRef<Input>) -> Option<Vec<u8>> {
        let input_ref = input_ref.get_untracked().unwrap();
        let file = input_ref.files()?.get(0)?;
        input_ref.set_value("");
        let file = gloo::file::Blob::from(file);
        let data: Vec<u8> = gloo::file::futures::read_as_bytes(&file).await.ok()?;
        Some(data)
    }

    let input_ref = create_node_ref::<Input>();
    let state = State::expect();
    let on_file_selection = move |_| {
        spawn_local(async move {
            // Backup and then delete the current database.
            let backup = indexeddb::export().await.ok();
            if let Err(err) = indexeddb::delete_all().await {
                error!("Unable to delete database: {err}");
            }
            let combined_data = get_file(input_ref)
                .await
                .and_then(|new| Some((backup?, new)));
            if let Some((backup, new)) = combined_data {
                match indexeddb::import(new).await {
                    Ok(_) => Toast::show("import success", "successfully imported database"),
                    Err(err) => {
                        logging::error!("Import error: {err}");
                        indexeddb::import(backup).await.unwrap_or_default();
                        Toast::show(
                            "import failure",
                            "failed to import user file, rolled back to previous database",
                        );
                    }
                }
            }
            update_db_size(state).await;
        });
    };

    view! {
        <div class= "flex gap-2 text-yellow-300 items-center">
            <div class= "w-6 fill-yellow-500 stroke-transparent" inner_html=icons::CAUTION />
            <div class= "w-12 grow">
                "Importing a file will completely replace (and therefore delete) the current database."
            </div>
        </div>
        <label class= "btn text-center bg-red-800" for= "import-file">
            "IMPORT"
        </label>
        <input
            class= "hidden"
            type= "file"
            id= "import-file"
            accept= ".bin"
            on:change=on_file_selection
            node_ref=input_ref
        />
    }
}

fn delete_database() -> impl IntoView {
    let state = State::expect();
    let delete = move || {
        spawn_local(async move {
            let result = indexeddb::delete_all().await;
            if let Err(err) = result {
                error!("Unable to delete database: {err}");
            }
            update_db_size(state).await;
        });
    };
    let disabled = State::slice(|state| state.db_size == 0);

    view! {
        <div class= "flex gap-2 text-yellow-300 items-center">
            <div class= "w-6 fill-yellow-500 stroke-transparent" inner_html=icons::CAUTION />
            <div class= "w-12 grow">
            "Once a database has been deleted it can only be recovered by a valid export file, proceed with caution."
            </div>
        </div>
        <ConfirmButton
            location=RevLocation::SettingDatabase
            on_click=delete
            disabled=disabled
            confirm_class= "btn bg-red-800"
        >
            "DELETE DATABASE"
        </ConfirmButton>
    }
}

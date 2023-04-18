use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-cover items-center justify-center text-center">
            <h1> "Wayfarer" </h1>
            <div class= "grid my-8 divide-y-2 divide-amber-600 w-fit">
                <div class= "flex flex-col space-y-2 items-center pb-2">
                    <Card title= "Create Character" link= "/char-creation" />
                    <Card title= "FAQ" link= "/coming-soon" />
                </div>
                <div class= "flex flex-col space-y-2 items-center pt-2">
                    <Card title= "Classes" link= "class" />
                    <Card title= "Spellbook" link= "/coming-soon" />
                    <Card title= "Combat" link= "/coming-soon" />
                    <Card title= "Adventuring" link= "/coming-soon" />
                    <Card title= "Spellcasting" link= "/coming-soon" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Card<T>(cx: Scope, title: T, link: T) -> impl IntoView
where
    T: AsRef<str>,
{
    let link = link.as_ref().to_string();
    let title = title.as_ref().to_string();

    view! { cx,
        <A href=link>
            <div class="p-2 bg-sky-800 rounded w-40">
                 {title}
            </div>
        </A>
    }
}

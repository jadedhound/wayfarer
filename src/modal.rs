use leptos::*;

#[component]
pub fn MiddleModal(cx: Scope, children: Children, hidden: ReadSignal<bool>) -> impl IntoView {
    view! {
        cx,
        <div class="relative z-10" hidden=move || hidden.get()>
          <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
          <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center items-center p-0">
              <div class="relative transform overflow-hidden rounded-lg bg-red-800 text-left shadow-xl transition-all my-8 w-full max-w-lg">
                {children(cx)}
              </div>
            </div>
          </div>
        </div>
    }
}

use leptos::*;

use crate::pc::realm::Follower;

impl IntoView for &Follower {
    fn into_view(self) -> View {
        let stats = self
            .stats
            .iter_enum()
            .map(|(stat, x)| {
                view! {
                    <div class= "py-1"> { format!("{stat} +{x}") } </div>
                }
            })
            .collect_view();
        view! {
            <div class= "flex flex-col text-left">
                <h5 class= ""> { &self.name } </h5>
                <div class= "italic font-tight text-sm"> { format!("LEVEL {}", self.level) } </div>
                <div class= "grid grid-cols-2 folstat-border border-purple-500 text-center">
                    { stats }
                </div>
            </div>
        }
        .into_view()
    }
}

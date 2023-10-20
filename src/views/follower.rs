use leptos::*;

use crate::icons;
use crate::pc::realm::{Follower, FollowerStat};

impl IntoView for &Follower {
    fn into_view(self) -> View {
        let hearts = self.stats.get(FollowerStat::Health);
        let expertise = self.stats.get(FollowerStat::Expertise);
        let inventory = self.stats.get(FollowerStat::Inventory);
        let morale = self.stats.get(FollowerStat::Morale);
        view! {
            <div class= "flex flex-col text-left">
                <h5 class= ""> { &self.name } </h5>
                <div class= "italic font-tight text-sm"> { format!("LEVEL {}", self.level) } </div>
                <div class= "grid grid-cols-2 folstat-border border-purple-500 text-center">
                    <Hearts hearts />
                    <div class= "py-1">
                        { format!("{} DC {}", FollowerStat::Morale, 10 - morale) }
                    </div>
                    <div class= "py-1">
                        { format!("{} +{}", FollowerStat::Expertise, expertise) }
                    </div>
                    <div class= "py-1">
                        { format!("{} +{}", FollowerStat::Inventory, inventory) }
                    </div>
                </div>
            </div>
        }
        .into_view()
    }
}

#[component]
fn Hearts(hearts: i32) -> impl IntoView {
    let hearts = (0..hearts + 1)
        .map(|_| {
            view! { <div class= "w-4" inner_html=icons::HEART /> }
        })
        .collect_view();
    view! {
        <div class= "flex justify-center">
            <div class= "grid grid-cols-5 items-center gap-1 py-1 fill-red-500 stroke-red-500">
                { hearts }
            </div>
        </div>
    }
}

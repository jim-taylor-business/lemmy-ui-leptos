use leptos::*;

use crate::ui::components::common::about::About;

#[component]
pub fn CommunitiesActivity() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <About /* site_signal=RwSignal::new(None)  *//>
      <h2 class="p-6 text-2xl">"Communities placeholder page"</h2>
    </main>
  }
}

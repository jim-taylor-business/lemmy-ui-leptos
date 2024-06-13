use leptos::*;

use crate::ui::components::common::about::About;

#[component]
pub fn CommunitiesActivity() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <About />
    </main>
  }
}

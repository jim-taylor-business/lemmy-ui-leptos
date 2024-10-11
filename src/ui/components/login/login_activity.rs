use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginActivity() -> impl IntoView {
  view! {
    <main class="p-3 mx-auto max-w-screen-md">
      <LoginForm />
    </main>
  }
}

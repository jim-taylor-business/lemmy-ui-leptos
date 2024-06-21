use leptos::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <div class="card w-full bg-base-300 text-base-content mb-3">
      <figure>
        <div class="card-body bg-accent">
          <h2 class="card-title text-info-content">"About this site"</h2>
        </div>
      </figure>
      <div class="card-body">
        <p>"This is a technical demo and proof of concept of the technical objectives specified on my "<a class="link" href="//github.com/jim-taylor-business/lemmy-ui-leptos#objectives">"Lemmy UI Leptos homepage"</a>"."</p>
        <p>"It is also intended to be near feature complete with the homepage functionality of "<a class="link" href="//lemmy.world">"Lemmy world"</a>", and near issue free."</p>
        <p>"This site is not affiliated with Lemmy World in any way."</p>
      </div>
    </div>
  }
}
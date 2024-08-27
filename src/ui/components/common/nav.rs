use crate::{
  cookie::{remove_cookie, set_cookie},
  errors::{message_from_error, LemmyAppError, LemmyAppErrorType},
  i18n::*,
  lemmy_client::*,
  ui::components::common::icon::{Icon, IconType::*},
  OnlineSetter,
};
use ev::MouseEvent;
use lemmy_api_common::{
  lemmy_db_schema::source::site::Site, lemmy_db_views::structs::SiteView, site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
use web_sys::SubmitEvent;

#[server(LogoutFn, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use leptos_actix::redirect;
  let result = LemmyClient.logout().await;
  match result {
    Ok(_o) => {
      let r = remove_cookie("jwt").await;
      match r {
        Ok(_o) => {
          redirect("/");
          Ok(())
        }
        Err(e) => {
          redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
          Ok(())
        }
      }
    }
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[server(ChangeLangFn, "/serverfn")]
pub async fn change_lang(lang: String) -> Result<(), ServerFnError> {
  let _ = set_cookie(
    "i18n_pref_locale",
    &lang.to_lowercase(),
    &core::time::Duration::from_secs(604800),
  )
  .await;
  Ok(())
}

#[server(ChangeThemeFn, "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  use leptos_actix::redirect;
  let r = set_cookie("theme", &theme, &core::time::Duration::from_secs(604800)).await;
  match r {
    Ok(_o) => Ok(()),
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[component]
pub fn TopNav(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();
  // let ssr_error = RwSignal::new::<Option<(LemmyAppError, Option<RwSignal<bool>>)>>(None);

  // if let Some(Err(e)) = site_signal.get() {
  //   ssr_error.set(Some((e, None)));
  // }

  let query = use_query_map();

  let ssr_query_error = move || {
    serde_json::from_str::<LemmyAppError>(&query.get().get("error").cloned().unwrap_or("".into()))
      .ok()
      .map(|e| (e, None::<Option<RwSignal<bool>>>))
    // .unwrap_or((0usize, None))
  };

  // let ssr_error = move || query.with(|params| params.get("error").cloned());

  // if let Some(e) = ssr_error() {
  //   if !e.is_empty() {
  //     let r = serde_json::from_str::<LemmyAppError>(&e[..]);

  //     match r {
  //       Ok(e) => {
  //         error.set(Some((e, None)));
  //       }
  //       Err(_) => {
  //         logging::error!("error decoding error - log and ignore in UI?");
  //       }
  //     }
  //   }
  // }

  let user = expect_context::<RwSignal<Option<bool>>>();

  let logout_action = create_server_action::<LogoutFn>();

  let on_logout_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let result = LemmyClient.logout().await;
        match result {
          Ok(_o) => {
            let _ = remove_cookie("jwt").await;
            user.set(Some(false));
          }
          Err(e) => {
            logging::warn!("logout error {:#?}", e);
            error.update(|es| es.push(Some((e, None))));
            // error.set(Some((e, None)));
          }
        }
      },
    );
  };

  // let site_signal = RwSignal::new::<Option<Result<GetSiteResponse, LemmyAppError>>>(None);

  let ssr_unread = Resource::new(
    move || (user.get()),
    move |user| async move {
      let result =
      //  if user == Some(true) {
        LemmyClient.unread_count().await
      // } else {
        // LemmyClient.unread_count().await;
      //   Err(LemmyAppError {
      //     error_type: LemmyAppErrorType::MissingToken,
      //     content: "".into(),
      //   })
      // }
      ;

      // logging::log!("{:#?}", result);

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          // error.set(Some((e.clone(), None)));
          Err(e)
        }
      }
    },
  );

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let ui_online = expect_context::<RwSignal<OnlineSetter>>();
  let theme_action = create_server_action::<ChangeThemeFn>();

  let on_theme_submit = move |theme_name: &'static str| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      let _res = create_local_resource(
        move || theme_name.to_string(),
        move |t| async move {
          let _ = set_cookie("theme", &t, &core::time::Duration::from_secs(604800)).await;
        },
      );
      ui_theme.set(Some(theme_name.to_string()));
    }
  };

  let lang_action = create_server_action::<ChangeLangFn>();

  let on_lang_submit = move |lang: Locale| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      i18n.set_locale(lang);
    }
  };

  view! {
    <nav class="navbar container mx-auto flex sticky top-0 bg-base-100 z-[1]">
      <div class="navbar-start">
        // <div class="dropdown"><div tabindex="0" role="button" class="btn btn-ghost lg:hidden"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16"></path></svg></div> <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"><li><button>Item 1</button></li> <li><button>Parent</button> <ul class="p-2 bg-base-100 w-40"><li><button>Submenu 1</button></li> <li><button>Submenu 2</button></li></ul></li> <li><button>Item 3</button></li></ul></div>
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              {move || {
                  if let Some(Ok(GetSiteResponse { site_view: SiteView { site: Site { icon: Some(i), .. }, .. },  .. })) = site_signal.get() {
                      view! { <img class="h-8" src={ i.inner().to_string() } /> }
                  } else {
                      view! { <img class="h-8" src="/lemmy.svg" /> }
                  }
              }}
              <span class="hidden lg:flex">
              {move || {
                  if let Some(Ok(m)) = site_signal.get() {
                      m.site_view.site.name
                  } else {
                      "Lemmy".to_string()
                  }
              }}
              </span>
            </A>
          </li>
          <li class="hidden lg:flex">
            <A href="/communities" class="text-md">
              {t!(i18n, communities)}
            </A>
          </li>
          <li class="hidden lg:flex">
            <A href="/create_post" class="text-md pointer-events-none text-base-content/50">
              {t!(i18n, create_post)}
            </A>
          </li>
          <li class="hidden lg:flex">
            <A href="/create_community" class="text-md pointer-events-none text-base-content/50">
              {t!(i18n, create_community)}
            </A>
          </li>
          <li class="hidden lg:flex">
            <a href="//join-lemmy.org/donate">
              <span title="t!(i18n, donate)">
                <Icon icon=Donate/>
              </span>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          // <li class="hidden lg:flex">
          //   <A href="/search" class="pointer-events-none text-base-content/50">
          //     <span title="t!(i18n, search)">
          //       <Icon icon=Search/>
          //     </span>
          //   </A>
          // </li>
          <li class="z-[1] hidden lg:flex">
            <details>
              <summary><Icon icon=Translate /></summary>
              <ul>
                <li>
                  <ActionForm action=lang_action on:submit=on_lang_submit(Locale::fr)>
                    <input type="hidden" name="lang" value="FR"/>
                    <button type="submit">"FR"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=lang_action on:submit=on_lang_submit(Locale::en)>
                    <input type="hidden" name="lang" value="EN"/>
                    <button type="submit">"EN"</button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <li class="z-[1] hidden lg:flex">
            <details>
              <summary><Icon icon=Palette /></summary>
              <ul>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("dark")>
                    <input type="hidden" name="theme" value="dark"/>
                    <button type="submit">"Dark"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("light")>
                    <input type="hidden" name="theme" value="light"/>
                    <button type="submit">"Light"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("retro")>
                    <input type="hidden" name="theme" value="retro"/>
                    <button type="submit">"Retro"</button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <li>
            <A href="/notifications">
              <span class="flex flex-row items-center">
                {move || {
                  let v = error.get();
                  if v.len() > 0 {
                    view! {
                      <div class="badge badge-error badge-xs"> { v.len() } </div>
                    }
                  } else {
                    view! {
                      <div class="hidden"></div>
                    }
                  }
                }}


                <span>
                {move || {
                  if ui_online.get().0 {
                    view! {
                      <div class="absolute top-0 badge badge-success badge-xs"></div>
                    }
                  } else {
                    view! {
                      <div class="absolute top-0 badge badge-warning badge-xs"></div>
                    }
                  }
                }}


                <Icon icon=Notifications/>
                </span>

                <Transition fallback=|| {}>
                  {move || {
                      ssr_unread
                          .get()
                          .map(|u| {
                              if let Ok(c) = u {
                                view! {
                                  <div class="badge badge-primary badge-xs"> { c.replies + c.mentions + c.private_messages } </div>
                                }
                              } else {
                                view! {
                                  <div class="hidden"></div>
                                }
                              }
                          })
                  }}

                </Transition>


              </span>
            </A>
          </li>
          <Show
            when=move || {
                if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = site_signal.get() {
                    true
                } else {
                    false
                }
            }

            fallback=move || {
                view! {
                  <li>
                    <A href="/login">{t!(i18n, login)}</A>
                  </li>
                  <li class="hidden lg:flex">
                    <A href="/signup" class="pointer-events-none text-base-content/50">{t!(i18n, signup)}</A>
                  </li>
                }
            }
          >
            <li>
              <details>
                <summary>
                  {move || {
                      if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = site_signal.get()
                      {
                          m.local_user_view
                              .person
                              .display_name
                              .unwrap_or(m.local_user_view.person.name)
                      } else {
                          String::default()
                      }
                  }}

                </summary>
                <ul class="z-10">
                  <li>
                    <A on:click=move |e: MouseEvent| {
                      if e.ctrl_key() && e.shift_key() {
                        e.stop_propagation();
                        if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = site_signal.get()
                        {
                          let _ = window().location().set_href(&format!("//lemmy.world/u/{}", m.local_user_view.person.name));
                        }
                      }
                    } href=move || {
                        format!(
                            "/u/{}",
                            if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = site_signal
                                .get()
                            {
                                m.local_user_view.person.name
                            } else {
                                String::default()
                            },
                        )
                    }>{t!(i18n, profile)}</A>
                  </li>
                  <li>
                    <A href="/settings">{t!(i18n, settings)}</A>
                  </li>
                  <div class="divider my-0"></div>
                  <li>
                    <ActionForm action=logout_action on:submit=on_logout_submit>
                      <button type="submit">{t!(i18n, logout)}</button>
                    </ActionForm>
                  </li>
                </ul>
              </details>
            </li>
          </Show>
        </ul>
      </div>
    </nav>
    // <Show
    //   when=move || error.get().is_some()
    //   fallback=move || {
    //       view! { <div class="hidden"></div> }
    //   }
    // >

    // {move || {
    //   site_signal.get()
    //         .map(|res| {

    //           if let Err(err) = res {
    //             view! {
    //               <div class="container mx-auto alert alert-error mb-8">
    //                 <span>"S" {message_from_error(&err)} " - " {err.content}</span>
    //                 <div>
    //                   <A href=use_location().pathname.get() class="btn btn-sm"> "Retry" </A>
    //                 </div>
    //               </div>
    //             }
    //           } else {
    //             view! {
    //               <div class="hidden" />
    //             }

    //           }
    //         })
    // }}

    {move || {
        ssr_query_error()
            .map(|err| {
                let mut query_params = query.get();
                query_params.remove("error".into());
                view! {
                  <div class="container mx-auto alert alert-error mb-8">
                    <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
                    <div>
                      <A class="btn btn-sm" href=format!("./?{}", &query_params.to_query_string())> "Clear" </A>
                    </div>
                  </div>
                }
            })
    }}

    // {move || {
    //     error.get()
    //       .map(|err| {
    //           view! {
    //             <div class="container mx-auto alert alert-error mb-8">
    //               <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
    //               <div>
    //                 <Show when=move || { if let Some(r) = err.1 { true } else { false } } /* let:r */ fallback=|| {}>
    //                   <button on:click=move |_| { if let Some(r) = err.1 { r.set(!r.get()); } else { } } class="btn btn-sm"> "Retry" </button>
    //                 </Show>
    //                 <button class="btn btn-sm" on:click=move |_| { error.set(None); }> "Clear" </button>
    //               </div>
    //             </div>
    //           }
    //       })
    // }}

    // </Show>
  }
}

#[component]
pub fn BottomNav(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let i18n = use_i18n();
  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");
  const GIT_HASH: std::option::Option<&'static str> = option_env!("GIT_HASH");

  view! {
    <nav class="container navbar mx-auto hidden lg:flex">
      <div class="navbar-start w-auto"></div>
      <div class="navbar-end grow w-auto">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <a href="//github.com/jim-taylor-business/lemmy-ui-leptos/releases" class="text-md">
              "FE: "
              {FE_VERSION} "." {GIT_HASH}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "BE: "
              {move || {
                  if let Some(Ok(m)) = site_signal.get() { m.version } else { "Lemmy".to_string() }
              }}
            </a>
          </li>
          <li>
            <A href="/modlog" class="text-md pointer-events-none text-base-content/50">
              {t!(i18n, modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="text-md pointer-events-none text-base-content/50">
              {t!(i18n, instances)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/docs/en/index.html" class="text-md">
              {t!(i18n, docs)}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet" class="text-md">
              {t!(i18n, code)}
            </a>
          </li>
          <li>
            <a href="//join-lemmy.org" class="text-md">
              "join-lemmy.org"
            </a>
          </li>
        </ul>
      </div>
    </nav>
  }
}

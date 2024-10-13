use crate::{
  // cookie::{remove_cookie, set_cookie},
  errors::{message_from_error, LemmyAppError},
  i18n::*,
  lemmy_client::*,
  ui::components::common::icon::{Icon, IconType::*},
  NotificationsRefresh,
  OnlineSetter,
  UriSetter,
};
use codee::string::FromToStringCodec;
use ev::MouseEvent;
use lemmy_api_common::{
  lemmy_db_schema::source::site::Site, lemmy_db_views::structs::SiteView, person::GetUnreadCountResponse, site::GetSiteResponse,
};
use leptos::*;
use leptos_dom::helpers::IntervalHandle;
use leptos_router::*;
use leptos_use::{use_cookie_with_options, use_document_visibility, SameSite, UseCookieOptions};
use web_sys::{SubmitEvent, VisibilityState};

#[server(LogoutFn, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use leptos_actix::redirect;
  let result = LemmyClient.logout().await;
  match result {
    Ok(_o) => {
      let (_, set_auth_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
        "jwt",
        UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
      );
      set_auth_cookie.set(None);
      // let r = remove_cookie("jwt").await;
      // match r {
      //   Ok(_o) => {
      //     redirect("/");
      Ok(())
      //   }
      //   Err(e) => {
      //     redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      //     Ok(())
      //   }
      // }
    }
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[server(ChangeLangFn, "/serverfn")]
pub async fn change_lang(lang: String) -> Result<(), ServerFnError> {
  let (_, set_locale_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
    "i18n_pref_locale",
    UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
  );
  set_locale_cookie.set(Some(lang.to_lowercase()));
  // let _ = set_cookie("i18n_pref_locale", &lang.to_lowercase(), &core::time::Duration::from_secs(604800)).await;
  Ok(())
}

#[server(ChangeThemeFn, "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  let (_, set_theme_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
    "theme",
    UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
  );
  set_theme_cookie.set(Some(theme));
  Ok(())
}

#[component]
pub fn TopNav(ssr_site: Resource<Option<bool>, Result<GetSiteResponse, LemmyAppError>>) -> impl IntoView {
  let i18n = use_i18n();

  let (_, set_theme_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
    "theme",
    UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
  );

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

  let authenticated = expect_context::<RwSignal<Option<bool>>>();
  let notifications_refresh = expect_context::<RwSignal<NotificationsRefresh>>();
  let uri = expect_context::<RwSignal<UriSetter>>();

  let logout_action = create_server_action::<LogoutFn>();

  let refresh = RwSignal::new(true);

  let unread_visibility: RwSignal<Option<Signal<VisibilityState>>> = RwSignal::new(None);
  let unread_effect: RwSignal<Option<Effect<()>>> = RwSignal::new(None);
  let unread_interval: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);

  // watch(deps, callback, immediate)

  let _unread_effect = Effect::new(move |_| match authenticated.get() {
    Some(true) => {
      // if let Some(v) = unread_visibility.get() {
      unread_visibility.set(Some(use_document_visibility()));

      // #[cfg(not(feature = "ssr"))]
      // let e = ;
      unread_effect.set(Some(Effect::new(move |_| match unread_visibility.get().unwrap().get() {
        VisibilityState::Visible => {
          refresh.update(|b| *b = !*b);
        }
        VisibilityState::Hidden => {}
        _ => {}
      })));

      // #[cfg(not(feature = "ssr"))]
      // let h = ;
      unread_interval.set(
        set_interval_with_handle(
          move || match unread_visibility.get().unwrap().get() {
            VisibilityState::Visible => {
              refresh.update(|b| *b = !*b);
            }
            VisibilityState::Hidden => {}
            _ => {}
          },
          std::time::Duration::from_millis(30000),
        )
        .ok(),
      );
    }
    _ => {
      if let Some(i) = unread_interval.get() {
        logging::log!("1");
        i.clear();
        unread_interval.set(None);
      }
      if let Some(e) = unread_effect.get() {
        logging::log!("2");
        e.dispose();
        unread_effect.set(None);
      }
      if let Some(_v) = unread_visibility.get() {
        logging::log!("3");
        unread_visibility.set(None);
      }
    }
  });

  // // #[cfg(not(feature = "ssr"))]
  // let visibility = use_document_visibility();

  // // #[cfg(not(feature = "ssr"))]
  // let _e = Effect::new(move |_| match visibility.get() {
  //   VisibilityState::Visible => {
  //     refresh.update(|b| *b = !*b);
  //   }
  //   VisibilityState::Hidden => {}
  //   _ => {}
  // });

  // // #[cfg(not(feature = "ssr"))]
  // let h = set_interval_with_handle(
  //   move || match visibility.get() {
  //     VisibilityState::Visible => {
  //       refresh.update(|b| *b = !*b);
  //     }
  //     VisibilityState::Hidden => {}
  //     _ => {}
  //   },
  //   std::time::Duration::from_millis(30000),
  // )
  // .ok();

  let on_logout_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let result = LemmyClient.logout().await;
        match result {
          Ok(_o) => {
            // #[cfg(not(feature = "ssr"))]
            // if let Some(h) = h {
            //   h.clear();
            // }
            let (_, set_auth_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
              "jwt",
              UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
            );
            set_auth_cookie.set(None);
            authenticated.set(Some(false));
          }
          Err(e) => {
            logging::warn!("logout error {:#?}", e);
            error.update(|es| es.push(Some((e, None))));
          }
        }
      },
    );
  };

  let logged_in = Signal::derive(move || {
    if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = ssr_site.get() {
      Some(true)
    } else {
      Some(false)
    }
  });

  let ssr_unread = Resource::new(
    move || (refresh.get(), logged_in.get(), notifications_refresh.get()),
    move |(_refresh, logged_in, _notifications_refresh)| async move {
      let result = if logged_in == Some(true) {
        LemmyClient.unread_count().await
      } else {
        Ok(GetUnreadCountResponse {
          replies: 0,
          mentions: 0,
          private_messages: 0,
        })
      };

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          error.update(|es| es.push(Some((e.clone(), None))));
          Err(e)
        }
      }
    },
  );

  let online = expect_context::<RwSignal<OnlineSetter>>();
  let theme_action = create_server_action::<ChangeThemeFn>();

  let on_theme_submit = move |theme_name: &'static str| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      set_theme_cookie.set(Some(theme_name.to_string()));
    }
  };

  let lang_action = create_server_action::<ChangeLangFn>();

  let on_lang_submit = move |lang: Locale| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      i18n.set_locale(lang);
    }
  };

  let on_navigate_login = move |ev: SubmitEvent| {
    ev.prevent_default();
    let l = use_location();
    uri.set(UriSetter(format!("{}{}", l.pathname.get(), l.query.get().to_query_string())));
    use_navigate()("/login", NavigateOptions::default());
  };

  view! {
    <nav class="container flex sticky top-0 mx-auto navbar bg-base-100 z-[1]">
      <div class="navbar-start">
        // <div class="dropdown"><div tabindex="0" role="button" class="btn btn-ghost lg:hidden"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16"></path></svg></div> <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"><li><button>Item 1</button></li> <li><button>Parent</button> <ul class="p-2 bg-base-100 w-40"><li><button>Submenu 1</button></li> <li><button>Submenu 2</button></li></ul></li> <li><button>Item 3</button></li></ul></div>
        <ul class="flex-nowrap items-center menu menu-horizontal">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              {move || {
                if let Some(Ok(GetSiteResponse { site_view: SiteView { site: Site { icon: Some(i), .. }, .. }, .. })) = ssr_site.get() {
                  view! { <img class="h-8" src={i.inner().to_string()} /> }
                } else {
                  view! { <img class="h-8" src="/lemmy.svg" /> }
                }
              }}
              <span class="hidden lg:flex">
                {move || { if let Some(Ok(m)) = ssr_site.get() { m.site_view.site.name } else { "Lemmy".to_string() } }}
              </span>
            </A>
          </li>
          // <li class="hidden lg:flex">
          // <A href="/communities" class="text-md">
          // {t!(i18n, communities)}
          // </A>
          // </li>
          // <li class="hidden lg:flex">
          // <A href="/create_post" class="text-md pointer-events-none text-base-content/50">
          // {t!(i18n, create_post)}
          // </A>
          // </li>
          <li class="hidden lg:flex">
            <A href="/communities" class="text-md">
              {t!(i18n, create_community)}
            </A>
          </li>
          <li class="hidden lg:flex">
            <a title={t!(i18n, donate)} href="//ko-fi.com/fhfworld">
              <Icon icon={Donate} />
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="flex-nowrap items-center menu menu-horizontal">
          <li class="hidden lg:flex">
            <A href="/search" class="pointer-events-none text-base-content/50">
              <span title="t!(i18n, search)">
                <Icon icon={Search} />
              </span>
            </A>
          </li>
          <li class="hidden lg:flex z-[1]">
            <details>
              <summary>
                <Icon icon={Translate} />
              </summary>
              <ul>
                <li>
                  <ActionForm class="p-0" action={lang_action} on:submit={on_lang_submit(Locale::fr)}>
                    <input type="hidden" name="lang" value="FR" />
                    <button class="py-2 px-4" type="submit">
                      "FR"
                    </button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm class="p-0" action={lang_action} on:submit={on_lang_submit(Locale::en)}>
                    <input type="hidden" name="lang" value="EN" />
                    <button class="py-2 px-4" type="submit">
                      "EN"
                    </button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <li class="hidden lg:flex z-[1]">
            <details>
              <summary>
                <Icon icon={Palette} />
              </summary>
              <ul>
                <li>
                  <ActionForm class="p-0" action={theme_action} on:submit={on_theme_submit("dark")}>
                    <input type="hidden" name="theme" value="dark" />
                    <button class="py-2 px-4" type="submit">
                      "Dark"
                    </button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm class="p-0" action={theme_action} on:submit={on_theme_submit("light")}>
                    <input type="hidden" name="theme" value="light" />
                    <button class="py-2 px-4" type="submit">
                      "Light"
                    </button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm class="p-0" action={theme_action} on:submit={on_theme_submit("retro")}>
                    <input type="hidden" name="theme" value="retro" />
                    <button class="py-2 px-4" type="submit">
                      "Retro"
                    </button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <Transition fallback={|| {}}>
            {move || {
              ssr_unread
                .get()
                .map(|u| {
                  let unread = if let Ok(c) = u.clone() { format!(", {} unread", c.replies + c.mentions + c.private_messages) } else { "".into() };
                  view! {
                    <li title={move || {
                      format!(
                        "{}{}{}",
                        if error.get().len() > 0 { format!("{} errors, ", error.get().len()) } else { "".into() },
                        if online.get().0 { "app online" } else { "app offline" },
                        unread,
                      )
                    }}>
                      <A href="/notifications">
                        <span class="flex flex-row items-center">
                          {move || {
                            let v = error.get();
                            (v.len() > 0)
                              .then(move || {
                                let l = v.len();
                                view! { <div class="badge badge-error badge-xs">{l}</div> }
                              })
                          }}
                          <span>
                            {move || { (!online.get().0).then(move || view! { <div class="absolute top-0 badge badge-warning badge-xs" /> }) }}
                            <Icon icon={Notifications} />
                          </span>
                          {if let Ok(c) = u {
                            (c.replies + c.mentions + c.private_messages > 0)
                              .then(move || view! { <div class="badge badge-primary badge-xs">{c.replies + c.mentions + c.private_messages}</div> })
                          } else {
                            None
                          }}
                        </span>
                      </A>
                    </li>
                  }
                })
            }}
          </Transition>
          <Show
            when={move || { if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = ssr_site.get() { true } else { false } }}
            fallback={move || {
              view! {
                // let l = use_location();

                <li>
                  // <ActionForm action="/login" on:submit=|_| {}>
                  // <input type="hidden" name="uri" value=move || format!("{}{}", l.pathname.get(), l.query.get().to_query_string())/>
                  // <button type="submit">"lowgin"</button>
                  // </ActionForm>
                  // <Form action="/login" method="POST" on:submit=|_| {}>
                  // <input type="hidden" name="theme" value="retro"/>
                  // <button type="submit">"LOGIN"</button>
                  // </Form>
                  <form class="p-0" action="/login" method="POST" on:submit={on_navigate_login}>
                    <button class="py-2 px-4" type="submit">
                      {t!(i18n, login)}
                    </button>
                  </form>
                // <A href="/login">{t!(i18n, login)}</A>
                </li>
                <li class="hidden lg:flex">
                  <A href="/signup" class="pointer-events-none text-base-content/50">
                    {t!(i18n, signup)}
                  </A>
                </li>
              }
            }}
          >
            <li>
              <details>
                <summary>
                  {move || {
                    if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = ssr_site.get() {
                      m.local_user_view.person.display_name.unwrap_or(m.local_user_view.person.name)
                    } else {
                      String::default()
                    }
                  }}
                </summary>
                <ul class="z-10">
                  <li>
                    <A
                      on:click={move |e: MouseEvent| {
                        if e.ctrl_key() && e.shift_key() {
                          e.stop_propagation();
                          if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = ssr_site.get() {
                            let _ = window().location().set_href(&format!("//lemmy.world/u/{}", m.local_user_view.person.name));
                          }
                        }
                      }}
                      href={move || {
                        format!(
                          "/u/{}",
                          if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = ssr_site.get() {
                            m.local_user_view.person.name
                          } else {
                            String::default()
                          },
                        )
                      }}
                    >
                      {t!(i18n, profile)}
                    </A>
                  </li>
                  <li>
                    <A class="pointer-events-none text-base-content/50" href="/settings">
                      {t!(i18n, settings)}
                    </A>
                  </li>
                  <div class="my-0 divider" />
                  <li>
                    <ActionForm action={logout_action} on:submit={on_logout_submit}>
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
    // when=move || error.get().is_some()
    // fallback=move || {
    // view! { <div class="hidden"></div> }
    // }
    // >

    // {move || {
    // site_signal.get()
    // .map(|res| {

    // if let Err(err) = res {
    // view! {
    // <div class="container mx-auto alert alert-error mb-8">
    // <span>"S" {message_from_error(&err)} " - " {err.content}</span>
    // <div>
    // <A href=use_location().pathname.get() class="btn btn-sm"> "Retry" </A>
    // </div>
    // </div>
    // }
    // } else {
    // view! {
    // <div class="hidden" />
    // }

    // }
    // })
    // }}

    {move || {
      ssr_query_error()
        .map(|err| {
          let mut query_params = query.get();
          query_params.remove("error".into());
          view! {
            <div class="container mx-auto mb-8 alert alert-error">
              <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
              <div>
                <A class="btn btn-sm" href={format!("./?{}", &query_params.to_query_string())}>
                  "Clear"
                </A>
              </div>
            </div>
          }
        })
    }}
  }
}

#[component]
pub fn BottomNav(ssr_site: Resource<Option<bool>, Result<GetSiteResponse, LemmyAppError>>) -> impl IntoView {
  let i18n = use_i18n();
  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");
  const GIT_HASH: std::option::Option<&'static str> = option_env!("GIT_HASH");

  let version = move || {
    if let Some(Ok(m)) = ssr_site.get() {
      m.version
    } else {
      "Lemmy".to_string()
    }
  };

  view! {
    <nav class="container hidden mx-auto lg:flex navbar">
      <div class="w-auto navbar-start" />
      <div class="w-auto navbar-end grow">
        <ul class="flex-nowrap items-center menu menu-horizontal">
          <li>
            <a href="//github.com/jim-taylor-business/lemmy-ui-leptos/releases" class="text-md">
              "FE: "
              {FE_VERSION}
              "."
              {GIT_HASH}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "BE: "
              {move || version()}
            </a>
          </li>
          <li>
            <A href="/modlog" class="pointer-events-none text-md text-base-content/50">
              {t!(i18n, modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="pointer-events-none text-md text-base-content/50">
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

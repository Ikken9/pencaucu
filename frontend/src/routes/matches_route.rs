use leptos::*;
use leptos_router::*;
use crate::models::match_model::MatchData;
use crate::services::match_service;

#[component]
pub fn Match() -> impl IntoView {
    let params = use_params_map();
    let match_data = create_resource(
        move || params.get().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                &match_service::match_url(&id)
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading match..." }>
            {move || match_data.get().map(|match_data: Option<MatchData>| match match_data {
                None => view! { <div>"Error loading match data."</div> },
                Some(match_data) => view! {
                    <div class="match-view">
                        <h1>{format!("Match ID: {}", match_data.id)}</h1>
                        <p>{format!("Date: {}", match_data.date)}</p>
                        <p>{format!("Admin Email: {}", match_data.admin_email)}</p>
                    </div>
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Matches() -> impl IntoView {
    let matches_data = create_resource(
        || (),
        |_| async {
            &match_service::matches_url()
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading matches..." }>
            {move || matches_data.get().map(|matches| match matches {
                None => view! { <div>"Error loading matches."</div> },
                Some(matches) => view! {
                    <div class="matches-list">
                        <For each=move || matches.clone() key=|m| m.id>
                            {move |match_data| view! {
                                <div class="match-item">
                                    <A href=format!("/match/{}", match_data.id)>
                                        {format!("Match ID: {}", match_data.id)}
                                    </A>
                                    <p>{format!("Date: {}", match_data.date)}</p>
                                    <p>{format!("Admin Email: {}", match_data.admin_email)}</p>
                                </div>
                            }}
                        </For>
                    </div>
                }
            })}
        </Suspense>
    }
}
use leptos::*;
use leptos::leptos_dom::{error, log};
use reqwest::StatusCode;
use crate::Navbar;
use crate::services::{match_service, stadium_service, stage_service, team_service};

#[component]
pub fn UploadMatch() -> impl IntoView {
    let (upload_match_error, set_upload_match_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let upload_match_action =
        create_action(move |(team_name, faced_team_name, date, stage_name, stadium_id): &(String, String, String, String, String)| {
            {
                let team_name = team_name.clone();
                let faced_team_name = faced_team_name.clone();
                let date = date.clone();
                let stage_name = stage_name.clone();
                let stadium_id = stadium_id.clone();

                async move {
                    set_wait_for_response.update(|w| *w = true);
                    match match_service::upload_match(team_name, faced_team_name, date, stage_name, stadium_id).await {
                        Ok(res) => {
                            let status_code = res.status();
                            match status_code {
                                StatusCode::CONFLICT => {
                                    set_upload_match_error.set(Some(String::from("That match already exists")));
                                },
                                StatusCode::CREATED => {
                                    set_upload_match_error.set(None);
                                },
                                StatusCode::INTERNAL_SERVER_ERROR => {
                                    set_upload_match_error.set(Some(String::from("Something went wrong...")));
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            set_upload_match_error.set(Some(e.to_string()));
                        }
                    }
                    set_wait_for_response.update(|w| *w = false);
                }
            }

        });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <div class="p-3">
            <div class= "font-kanit text-xl font-bold italic text-zinc-300">
            UPLOAD A MATCH
            </div>
        </div>
        <UploadMatchForm
            action_label="Upload Match"
            action=upload_match_action
            error=upload_match_error.into()
            disabled
        />
        <Navbar/>
    }
}

#[component]
fn UploadMatchForm(
    action_label: &'static str,
    action: Action<(String, String, String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (team_name, set_team_name) = create_signal(String::new());
    let (faced_team_name, set_faced_team_name) = create_signal(String::new());
    let (date, set_date) = create_signal(String::new());
    let (stage, set_stage) = create_signal(String::new());
    let (stadium, set_stadium) = create_signal(String::new());

    let upload_match_action =
        move || action.dispatch((
            team_name.get(), faced_team_name.get(), date.get(), stage.get(), stadium.get()
        ));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || team_name.get().is_empty() || faced_team_name.get().is_empty() || date.get().is_empty() || stage.get().is_empty() || stadium.get().is_empty()
    });

    let stages_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching knockout stages...");
            let result = stage_service::get_knockout_stages().await;
            match result {
                Ok(stages) => {
                    log!("Successfully fetched knockout stages.");
                    Some(stages)
                }
                Err(e) => {
                    error!("Error fetching knockout stages: {:?}", e);
                    None
                }
            }
        },
    );

    let stadiums_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching stadiums...");
            let result = stadium_service::get_stadiums().await;
            match result {
                Ok(stadiums) => {
                    log!("Successfully fetched stadiums.");
                    Some(stadiums)
                }
                Err(e) => {
                    error!("Error fetching stadiums: {:?}", e);
                    None
                }
            }
        },
    );

    let teams_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching knockout stages...");
            let result = team_service::get_teams_names().await;
            match result {
                Ok(stages) => {
                    log!("Successfully fetched teams names.");
                    Some(stages)
                }
                Err(e) => {
                    error!("Error fetching teams names: {:?}", e);
                    None
                }
            }
        },
    );

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-zinc-300">Set Match Data</h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm p-4 sm:p-8">
                <form class="space-y-6" on:submit=|ev| ev.prevent_default()>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label for="date" class="block text-sm font-medium leading-6 text-zinc-300">Match Date</label>
                        <div class="mt-2">
                            <input id="date" name="date" type="datetime-local" min="2024-06-20T00:00" max="2024-07-14T23:59" autocomplete="match date" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 ring-gradient-to-r from-purple-500 via-purple-600 to-purple-700 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-purple-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    let val = event_target_value(&ev);
                                    set_date.update(|v| *v = val);
                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_date.update(|v| *v = val);
                                }
                            />
                        </div>
                    </div>
                    <div>
                        <label for="stage" class="block text-sm font-medium leading-6 text-zinc-300">"Knockout Stage"</label>
                        <div class="mt-2">
                            <select id="stage" name="stages" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_stage.update(|v| *v = val);
                                }
                            >
                                <option value="">Select a Knockout Stage</option>
                                {move || stages_data.get().map(|stages| {
                                    if let Some(stages_list) = stages {
                                        stages_list.iter().map(|s| {
                                            view! { <option value={s.name.clone()}>{s.name.clone()}</option> }
                                        }).collect::<Vec<_>>()
                                    } else {
                                        vec![view! { <option value="">Error loading stages</option> }]
                                    }
                                }).unwrap_or_else(|| vec![view! { <option value="">Error loading stages</option> }])}
                            </select>
                        </div>
                    </div>
                    <div>
                        <label for="stadium" class="block text-sm font-medium leading-6 text-zinc-300">"Stadium"</label>
                        <div class="mt-2">
                            <select id="stadium" name="stadiums" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_stadium.update(|v| *v = val);
                                }
                            >
                                <option value="">Select a Stadium</option>
                                {move || stadiums_data.get().map(|stadiums| {
                                    if let Some(stadiums_list) = stadiums {
                                        stadiums_list.iter().map(|s| {
                                            view! { <option value={s.name.clone()}>{s.name.clone()}</option> }
                                        }).collect::<Vec<_>>()
                                    } else {
                                        vec![view! { <option value="">Error loading stadiums</option> }]
                                    }
                                }).unwrap_or_else(|| vec![view! { <option value="">Error loading stadiums</option> }])}
                            </select>
                        </div>
                    </div>
                    <div>
                        <label for="team-name" class="block text-sm font-medium leading-6 text-zinc-300">"Team Name"</label>
                        <div class="mt-2">
                            <select id="team-name" name="team names" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_team_name.update(|v| *v = val);
                                }
                            >
                                <option value="">Select a Team Name</option>
                                {move || teams_data.get().map(|teams| {
                                    if let Some(teams_list) = teams {
                                        teams_list.iter().map(|t| {
                                            view! { <option value={t.clone()}>{t.clone()}</option> }
                                        }).collect::<Vec<_>>()
                                    } else {
                                        vec![view! { <option value="">Error loading teams</option> }]
                                    }
                                }).unwrap_or_else(|| vec![view! { <option value="">Error loading teams</option> }])}
                            </select>
                        </div>
                    </div>
                    <div>
                        <label for="faced-team-name" class="block text-sm font-medium leading-6 text-zinc-300">"Faced Team Name"</label>
                        <div class="mt-2">
                            <select id="faced-team-name" name="team names" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_faced_team_name.update(|v| *v = val);
                                }
                            >
                                <option value="">Select a Faced Team Name</option>
                                {move || teams_data.get().map(|teams| {
                                    if let Some(teams_list) = teams {
                                        teams_list.iter().map(|t| {
                                            view! { <option value={t.clone()}>{t.clone()}</option> }
                                        }).collect::<Vec<_>>()
                                    } else {
                                        vec![view! { <option value="">Error loading teams</option> }]
                                    }
                                }).unwrap_or_else(|| vec![view! { <option value="">Error loading teams</option> }])}
                            </select>
                        </div>
                    </div>
                    <div>
                        <button type="submit" class="flex w-full justify-center rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        prop:disabled=move || button_is_disabled.get()
                        on:click=move |_| upload_match_action()
                        >
                            {action_label}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
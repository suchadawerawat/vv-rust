
use leptos::*;
use leptos::logging::log;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    // Note: We don't usually include password_hash on the frontend
    pub created_at: String, // Or DateTime<Utc> if you handle it
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String, // Or DateTime<Utc>
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub completed: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(None::<String>)); // JWT Token Context

    view! {
        <Router>
            <Routes>
                <Route path="/register" view=Register/>
                <Route path="/login" view=Login/>
                <Route path="/" view=move || {
                    let token = use_context::<RwSignal<Option<String>>>();
                    match token.get() {
                        Some(_) => view! { <TaskList/> }.into_view(),
                        None => view! { <Navigate to="/login"/> }.into_view(),
                    }
                }/>
                <Route path="/tasks" view=TaskList/>
                <Route path="/tasks/:id" view=TaskDetail/>
            </Routes>
        </Router>
    }
}

#[component]
fn Register() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = useNavigate();
    let set_token = use_context::<RwSignal<Option<String>>>().expect("token context provided");

    let register_user = move |_| {
        let username = username.get();
        let password = password.get();
        spawn_local(async move {
            let res = reqwest::Client::new()
                .post("/api/auth/register")
                .json(&json!({"username": username, "password": password}))
                .send()
                .await
                .unwrap();

            if res.status().is_ok() {
                let body: Result<AuthResponse, _> = res.json().await;
                if let Ok(auth_response) = body {
                    set_token.set(Some(auth_response.token));
                    navigate("/").unwrap();
                }
            } else {
                let body: Result<ErrorResponse, _> = res.json().await;
                if let Ok(error_response) = body {
                    set_error.set(Some(error_response.error));
                } else {
                    set_error.set(Some("Registration failed".to_string()));
                }
            }
        });
    };

    view! {
        <h1>"Register"</h1>
        {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
        <input type="text" placeholder="Username" on:input=move |ev| set_username(event_target_value(&ev))/> <br/>
        <input type="password" placeholder="Password" on:input=move |ev| set_password(event_target_value(&ev))/> <br/>
        <button on:click=register_user>"Register"</button>
        <p>"Already have an account? "<A href="/login">"Login"</A></p>
    }
}

#[component]
fn Login() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = useNavigate();
    let set_token = use_context::<RwSignal<Option<String>>>().expect("token context provided");

    let login_user = move |_| {
        let username = username.get();
        let password = password.get();
        spawn_local(async move {
            let res = reqwest::Client::new()
                .post("/api/auth/login")
                .json(&json!({"username": username, "password": password}))
                .send()
                .await
                .unwrap();

            if res.status().is_ok() {
                let body: Result<AuthResponse, _> = res.json().await;
                if let Ok(auth_response) = body {
                    set_token.set(Some(auth_response.token));
                    navigate("/").unwrap();
                }
            } else {
                let body: Result<ErrorResponse, _> = res.json().await;
                if let Ok(error_response) = body {
                    set_error.set(Some(error_response.error));
                } else {
                    set_error.set(Some("Login failed".to_string()));
                }
            }
        });
    };

    view! {
        <h1>"Login"</h1>
        {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
        <input type="text" placeholder="Username" on:input=move |ev| set_username(event_target_value(&ev))/> <br/>
        <input type="password" placeholder="Password" on:input=move |ev| set_password(event_target_value(&ev))/> <br/>
        <button on:click=login_user>"Login"</button>
        <p>"Don't have an account? "<A href="/register">"Register"</A></p>
    }
}

#[component]
fn TaskList() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(Vec::<Task>::new());
    let (new_task, set_new_task) = create_signal(CreateTask::default());
    let (error, set_error) = create_signal(None::<String>);
    let token = use_context::<RwSignal<Option<String>>>().expect("token context provided");

    let fetch_tasks = move |_| {
        let token = token.get();
        spawn_local(async move {
            if let Some(token) = token {
                let res = reqwest::Client::new()
                    .get("/api/tasks")
                    .bearer_auth(token)
                    .send()
                    .await
                    .unwrap();

                if res.status().is_ok() {
                    let body: Result<Vec<Task>, _> = res.json().await;
                    if let Ok(task_list) = body {
                        set_tasks.set(task_list);
                    }
                } else {
                    let body: Result<ErrorResponse, _> = res.json().await;
                    if let Ok(error_response) = body {
                        set_error.set(Some(error_response.error));
                    } else {
                        set_error.set(Some("Failed to fetch tasks".to_string()));
                    }
                }
            } else {
                set_error.set(Some("Not authenticated".to_string()));
            }
        });
    };

    let create_new_task = move |_| {
        let new_task = new_task.get();
        let token = token.get();
        set_new_task(CreateTask::default()); // Clear the input
        spawn_local(async move {
            if let Some(token) = token {
                let res = reqwest::Client::new()
                    .post("/api/tasks")
                    .bearer_auth(token)
                    .json(&new_task)
                    .send()
                    .await
                    .unwrap();

                if res.status().is_created() {
                    fetch_tasks(()); // Refresh the task list
                } else {
                    let body: Result<ErrorResponse, _> = res.json().await;
                    if let Ok(error_response) = body {
                        set_error.set(Some(error_response.error));
                    } else {
                        set_error.set(Some("Failed to create task".to_string()));
                    }
                }
            } else {
                set_error.set(Some("Not authenticated".to_string()));
            }
        });
    };

    let update_task_completion = move |task_id: i64, completed: bool| {
        let token = token.get();
        spawn_local(async move {
            if let Some(token) = token {
                let res = reqwest::Client::new()
                    .put(&format!("/api/tasks/{}", task_id))
                    .bearer_auth(token)
                    .json(&json!({"completed": completed}))
                    .send()
                    .await
                    .unwrap();

                if res.status().is_ok() {
                    fetch_tasks(()); // Refresh the task list
                } else {
                    let body: Result<ErrorResponse, _> = res.json().await;
                    if let Ok(error_response) = body {
                        log!("{:?}", error_response);
                        set_error.set(Some(error_response.error));
                    } else {
                        set_error.set(Some("Failed to update task".to_string()));
                    }
                }
            } else {
                set_error.set(Some("Not authenticated".to_string()));
            }
        });
    };

    let delete_task_fn = move |task_id: i64| {
        let token = token.get();
        spawn_local(async move {
            if let Some(token) = token {
                let res = reqwest::Client::new()
                    .delete(&format!("/api/tasks/{}", task_id))
                    .bearer_auth(token)
                    .send()
                    .await
                    .unwrap();

                if res.status().is_no_content() {
                    fetch_tasks(()); // Refresh the task list
                } else {
                    let body: Result<ErrorResponse, _> = res.json().await;
                    if let Ok(error_response) = body {
                        set_error.set(Some(error_response.error));
                    } else {
                        set_error.set(Some("Failed to delete task".to_string()));
                    }
                }
            } else {
                set_error.set(Some("Not authenticated".to_string()));
            }
        });
    };

    on_mount(fetch_tasks);

    view! {
        <h1>"Tasks"</h1>
        {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
        <div>
            <input
                type="text"
                placeholder="Task Title"
                on:input=move |ev| set_new_task.update(|t| t.title = event_target_value(&ev))
                value=move || new_task.get().title
            />
            <input
                type="text"
                placeholder="Description (optional)"
                on:input=move |ev| set_new_task.update(|t| t.description = Some(event_target_value(&ev)))
                value=move || new_task.get().description.clone().unwrap_or_default()
            />
            <button on:click=create_new_task>"Add Task"</button>
        </div>
        <ul>
            <For
                each=move || tasks.get()
                key=|task| task.id
                let:task
            >
                <li>
                    <input
                        type="checkbox"
                        checked=move || task.completed
                        on:change=move |ev| update_task_completion(task.id, event_target_checked(&ev))
                    />
                    <span>{move || task.title.clone()}</span>
                    <button on:click=move |_| delete_task_fn(task.id)>"Delete"</button>
                </li>
            </For>
        </ul>
        <A href="/login">"Logout"</A>
    }
}

#[component]
fn TaskDetail() -> impl IntoView {
    let params = use_params_map();
    let task_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().ok());
    let (task, set_task) = create_signal(None::<Task>);
    let (error, set_error) = create_signal(None::<String>);
    let token = use_context::<RwSignal<Option<String>>>().expect("token context provided");

    let fetch_task = move |_| {
        if let Some(id) = task_id() {
            let token = token.get();
            spawn_local(async move {
                if let Some(token) = token {
                    let res = reqwest::Client::new()
                        .get(&format!("/api/tasks/{}", id))
                        .bearer_auth(token)
                        .send()
                        .await
                        .unwrap();

                    if res.status().is_ok() {
                        let body: Result<Task, _> = res.json().await;
                        if let Ok(task_data) = body {
                            set_task.set(Some(task_data));
                        }
                    } else if res.status().as_u16() == 404 {
                        set_error.set(Some("Task not found".to_string()));
                    } else {
                        let body: Result<ErrorResponse, _> = res.json().await;
                        if let Ok(error_response) = body {
                            set_error.set(Some(error_response.error));
                        } else {
                            set_error.set(Some("Failed to fetch task details".to_string()));
                        }
                    }
                } else {
                    set_error.set(Some("Not authenticated".to_string()));
                }
            });
        }
    };

    on_mount(fetch_task);

    view! {
        <h1>"Task Detail"</h1>
        {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
        {move || task.get().map(|task| view! {
            <p>"Title: "{task.title}</p>
            <p>"Description: "{task.description.unwrap_or_else(|| "No description".to_string())}</p>
            

<p>"Completed: "{if task.completed { "Yes" } else { "No" }}</p>
            <p>"Created At: "{task.created_at}</p>
            <p>"Updated At: "{task.updated_at}</p>
            <A href="/tasks">"Back to Tasks"</A>
        })}
        {move || task.get().is_none().then(|| error.get().map(|err| view! { <p class="error">{err}</p> }))}
    }
}

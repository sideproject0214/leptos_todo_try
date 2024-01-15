
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, PgConnection};

        pub async fn db() -> Result<PgConnection, ServerFnError> {
            Ok(PgConnection::connect("postgres://dev:password123@localhost:5432/todo").await?)
        }
    }
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError>{
    // this is just an example of how to access server context injected in the handlers
    let req = use_context::<actix_web::HttpRequest>();

    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }
    // use futures_util::stream::try_stream::TryStreamExt;
    use futures::TryStreamExt;
    let mut conn = db().await?;

    let mut todos = Vec::new();
    let mut rows =
        sqlx::query_as::<_, Todo>("select * from todos").fetch(&mut conn);
    
    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }
    
    Ok(todos)
}


#[server(AddTodo, "/api", "Cbor")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
/// Renders the home page of your application.

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done
    // during initial server-side rendering
    // if you navigate to the 404 page subsequently, the
    // status code will not be set because there is not a new
    // HTTP request to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}


// The struct name and path prefix arguments are optional.
#[server]
pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())?)
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/leptos_todo_try.css"/>
        <Router>
            <header>
                <h1>"My Tasks"</h1>
            </header>
            <main>
                <Routes>
                    <Route path="" view=Todos/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>();
    let delete_todo = create_server_action::<DeleteTodo>();
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = create_resource(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    view! {
        <div>
            <MultiActionForm
                // we can handle client-side validation in the on:submit event
                // leptos_router implements a `FromFormData` trait that lets you
                // parse deserializable types from form data and check them
                on:submit=move |ev| {
                    let data = AddTodo::from_event(&ev).expect("to parse form data");
                    if data.title == "nope!" {
                        ev.prevent_default();
                    }
                }

                action=add_todo
            >
                <label>"Add a Todo" <input type="text" name="title"/></label>
                <input type="submit" value="Add"/>
            </MultiActionForm>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    let existing_todos = {
                        move || {
                            todos
                                .get()
                                .map(move |todos| match todos {
                                    Err(e) => {
                                        view! {
                                            <pre class="error">"Server Error: " {e.to_string()}</pre>
                                        }
                                            .into_view()
                                    }
                                    Ok(todos) => {
                                        if todos.is_empty() {
                                            view! { <p>"No tasks were found."</p> }.into_view()
                                        } else {
                                            todos
                                                .into_iter()
                                                .map(move |todo| {
                                                    view! {
                                                        <li>
                                                            {todo.title} <ActionForm action=delete_todo>
                                                                <input type="hidden" name="id" value=todo.id/>
                                                                <input type="submit" value="X"/>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };
                    let pending_todos = move || {
                        submissions
                            .get()
                            .into_iter()
                            .filter(|submission| submission.pending().get())
                            .map(|submission| {
                                view! {
                                    <li class="pending">
                                        {move || submission.input.get().map(|data| data.title)}
                                    </li>
                                }
                            })
                            .collect_view()
                    };
                    view! { <ul>{existing_todos} {pending_todos}</ul> }
                }}

            </Transition>
        </div>
    }
}

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    id: u16,
    title: String,
    completed: bool,
}

// cfg_if! {
//     if #[cfg(feature= "ssr")]{
//         use sqlx::{postgres::PgPoolOptions, Pool,
// Postgres};

//         pub async fn db() -> Result<SqliteConnection,
// ServerFnError>{
// Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
//         }
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta
    // tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
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
    view! { <h1>Hello</h1> }
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

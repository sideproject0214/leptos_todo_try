use cfg_if::cfg_if;

mod app;

// 이 아래에서 가져오는 것들은 Cargo.toml에서
// [features][package.metadata.cargo-all-features]안에 활성화 해줘야 한다.

cfg_if! {
    if #[cfg(feature="ssr")]{
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use sqlx::{Pool, Postgres};
        use crate::app::*;

        // #[get("/style.scss")]
        // async fn css() -> impl Responder {
        //     actix_files::NamedFile::open_async("./public/style.scss").await
        // }

        pub struct AppState {
            pub db: Pool<Postgres>,
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()>{
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);

            let mut conn = db().await.expect("couldn't connect to DB");
            sqlx::migrate!("src/migrations").run(&mut conn).await.expect("Could not run sqlx migrations");

            HttpServer::new(move ||{
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                .wrap(middleware::Logger::default())
                // .app_data(web::Data::new(AppState {
                //     db: pool.clone(),
                // }))
                // .service(css)
                .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
                .service(Files::new("/", site_root))
                //.wrap(middleware::Compress::default()
            })
            .bind(addr)?
            .run()
            .await
        }

    } else {
        fn main(){

        }
    }
}

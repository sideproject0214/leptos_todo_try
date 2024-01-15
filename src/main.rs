use cfg_if::cfg_if;
mod app;

cfg_if! {
    if #[cfg(feature="ssr")]{
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use crate::app::*;

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style.css").await
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()>{
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);

            HttpServer::new(move ||{
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
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

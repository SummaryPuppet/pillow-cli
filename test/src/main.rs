use pillow::{
    http::*,
    templates::{Context, Template},
};

#[controller(method = "GET", path = "/")]
pub fn index() -> Response {
    Response::html("index")
}


#[controller(method = "GET", path = "/user")]
pub fn users() -> Response {
    let mut ctx = Context::new();

    ctx.insert("name", "SummaryPuppet");
    ctx.insert("id", &request.get_param("id"));

    Response::view(Template::Tera("users", "tera.html", ctx))
}

#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    // router.public();
    router.assets();

    router.add_route(route!(index {}));

    let server = Server::default();

    server.run(router).await;
}

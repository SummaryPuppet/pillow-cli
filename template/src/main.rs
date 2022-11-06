use pillow::router::Router;

fn main() {
    let mut app = Router::new();

    app.get("/", |_, response| response.view("index.html"));

    app.listen("5000")
}

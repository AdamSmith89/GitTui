mod panes;
mod app;
use app::App;

fn main() {
    let mut app = App::new();
    if let Err(e) = app.run() {
        println!("Error: {e}");
    }
}

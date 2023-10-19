mod utils;
mod input;
mod app;
mod update;
mod view;

use app::App;

// entry function
fn main() {
    yew::Renderer::<App>::new().render();
}

mod utils;
mod input;
mod app;
mod update;
mod view;
mod tests;

use app::App;

/// Entry function for webapp.
fn main() {
    yew::Renderer::<App>::new().render();
}

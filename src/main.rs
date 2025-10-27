mod app;
mod components;
mod pokemon;
mod theme;

use app::App;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}

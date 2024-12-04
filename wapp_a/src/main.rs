use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <main class="container">
                <h1>"Web App A"</h1>
            </main>
        }
    })
}

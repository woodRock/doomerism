use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Navigation - navigation.rs
/// ==========================
/// This is the navigation component. It is used to navigate between pages.
#[function_component(Navigation)]
pub fn navigation() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::Home);
    
    // Function to determine if a link is active
    let is_active = |route: Route| -> String {
        if route == current_route {
            "nav-item active".to_string()
        } else {
            "nav-item".to_string()
        }
    };

    html! {
        <nav class="navigation">
            <div class="nav-container">
                <div class="nav-logo">
                    <Link<Route> to={Route::Home} classes="logo-link">
                        <span class="logo-text">{"Rust App"}</span>
                    </Link<Route>>
                </div>
                
                <div class="nav-links">
                    <div class={is_active(Route::Home)}>
                        <Link<Route> to={Route::Home} classes="nav-link">
                            { "Home" }
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}
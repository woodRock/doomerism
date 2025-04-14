use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

/// ReturnHome - return_home.rs
/// ==========================
/// This is the return home button component.
#[function_component(ReturnHome)]
pub fn return_home() -> Html {
    html! {
        <Link<Route> to={Route::Home} classes="cybr-btn">
            { "Return to Main Grid " }
            <span class="cybr-btn__glitch">{ "HOME_/" }</span>
            <span class="cybr-btn__tag">{ "R:0" }</span>
        </Link<Route>>
    }
}
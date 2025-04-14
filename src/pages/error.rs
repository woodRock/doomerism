use crate::components::return_home::ReturnHome;
/// Error - error.rs
/// ===============
/// This is the error page. It is displayed when the user tries to access a page that does not exist.
use yew::prelude::*;

#[function_component(Error)]
pub fn error() -> Html {
    html! {
        <div class="error-container">
            <div class="glitch-container">
                <div class="glitch" data-text="404">{ "404" }</div>
            </div>
            <div class="error-message">
                <div class="scanline"></div>
                <p class="subtitle">{ "SYSTEM MALFUNCTION" }</p>
                <p class="message">{ "The neural pathway you're searching for doesn't exist in this consciousness." }</p>
                <div class="terminal">
                    <p class="terminal-text">{ ">> Error code: 404.PAGE_NOT_FOUND" }</p>
                    <p class="terminal-text">{ ">> System diagnostic: Route connectivity failure" }</p>
                    <p class="terminal-text">{ ">> Recommendation: Return to known coordinate" }</p>
                    <div class="blink">{ "█" }</div>
                </div>
                <div class="return-btn-container">
                    <ReturnHome />
                </div>
            </div>
        </div>
    }
}
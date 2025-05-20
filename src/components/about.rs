use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-white px-6">
            <div class="mb-6">
                <Link<Route> to={Route::Chat} classes="bg-blue-600 text-white px-4 py-2 rounded-full shadow hover:bg-blue-800">
                    {"← Back to Chat"}
                </Link<Route>>
            </div>
            <div class="text-center">
                <h1 class="text-3xl font-bold text-blue-700">{"👋 Welcome to YewChat!"}</h1>
                <p class="mt-4 text-lg text-gray-600">{"This is a creative WebSocket-based chat built with Yew in Rust."}</p>
                <p class="mt-2 text-sm text-gray-500">{"Made with 💙 for the creativity experiment (3.2)"}</p>
            </div>
        </div>
    }
}
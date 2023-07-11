use leptos::*;
use leptos_router::*;

#[component]
pub fn Boughts(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <A class="button" href="/">
                "الرئيسية"
            </A>
            <br/>
            <h1>"Boughts"</h1>
            <Outlet/>
        </div>
    }
}

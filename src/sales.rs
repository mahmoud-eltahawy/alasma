use leptos::*;
use leptos_router::*;

use models::backend_api::SellBill;

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    view!{cx,
	  <div>
	  <A class="button" href="/">"الرئيسية"</A><br/>
	  <h1>"Sales"</h1>
	  <Outlet/>
	</div>
    }
}

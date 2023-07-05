use leptos::*;
use leptos_router::*;

#[component]
pub fn Cargos(cx: Scope) -> impl IntoView {
    view!{cx,
	<div>
	  <A class="button" href="/">"الرئيسية"</A><br/>
	  <h1>"Cargos"</h1>
	  <Outlet/>
	</div>
    }
}

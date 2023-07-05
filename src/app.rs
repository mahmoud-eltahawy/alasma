use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{sales::Sales,boughts::Boughts,cargos::Cargos};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view!{cx,
        <Router>
          <section>
            <Routes>
              <Route path="/" view=|cx| view! {cx,<Home/>}/>
              <Route path="/sales" view=|cx| view! {cx,<Sales/>}/>
              <Route path="/boughts" view=|cx| view! {cx,<Boughts/>}/>
              <Route path="/cargos" view=|cx| view! {cx,<Cargos/>}/>
            </Routes>
          </section>
        </Router>
    }
}


#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view!{cx,
	<div>
	  <A class="button" href="/sales">"المبيعات"</A><br/>
	  <A class="button" href="/boughts">"المشتريات"</A><br/>
	  <A class="button" href="/cargos">"الاصناف"</A><br/>
	  <Outlet/>
	</div>
    }
}

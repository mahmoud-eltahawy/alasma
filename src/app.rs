use leptos::*;
use leptos_router::*;

use crate::{
    boughts::Boughts,
    cargos::Cargos,
    sales::{
	add::SaleSheetAdd,
	show::ShowSheet,
	Sales,
    },
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <section>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <Home/> }
                        }
                    />
                    <Route
                        path="/sales"
                        view=|cx| {
                            view! { cx, <Sales/> }
                        }
                    />
                    <Route
                        path="/sales/add"
                        view=|cx| {
                            view! { cx, <SaleSheetAdd/> }
                        }
                    />
                    <Route
                        path="/sales/show/:id"
                        view=|cx| {
                            view! { cx, <ShowSheet/> }
                        }
                    />
                    <Route
                        path="/boughts"
                        view=|cx| {
                            view! { cx, <Boughts/> }
                        }
                    />
                    <Route
                        path="/cargos"
                        view=|cx| {
                            view! { cx, <Cargos/> }
                        }
                    />
                </Routes>
            </section>
        </Router>
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <A class="button" href="/sales">
                "المبيعات"
            </A>
            <br/>
            <A class="button" href="/boughts">
                "المشتريات"
            </A>
            <br/>
            <A class="button" href="/cargos">
                "الاصناف"
            </A>
            <br/>
            <Outlet/>
        </div>
    }
}

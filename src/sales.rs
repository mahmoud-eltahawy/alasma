use std::str::FromStr;

use bigdecimal::ToPrimitive;
use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;

use models::backend_api::{
    Bill, Client, Company, Name, SellBill, SheetShearchParams, SheetType,
};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;
use uuid::Uuid;

pub mod add;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct SheetArgs {
    params: SheetShearchParams,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct IdArg {
    id: Uuid,
}

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    let (offset, set_offset) = create_signal(cx, 0 as i64);
    let (begin, set_begin) = create_signal(cx, None::<NaiveDate>);
    let (end, set_end) = create_signal(cx, None::<NaiveDate>);
    let (name, set_name) = create_signal(cx, None::<String>);

    let search_args = move || {
	let s = SheetArgs {
	    params: SheetShearchParams {
		offset: offset.get(),
		sheet_type: SheetType::Sells,
		name: name.get(),
		begin: begin.get(),
		end: end.get(),
	    },
	};
	log!("{:#?}",s);

	s
    };

    let bills = create_resource(
	cx,
	search_args,
	|value| async move {
        invoke::<_, Vec<Name>>("top_5_sheets", &value)
            .await
            .unwrap_or_default()
    });

    view! { cx,
        <section>
            <A class="right-corner" href="add">
                "+"
            </A>
            <A class="left-corner" href="/">
                "->"
            </A>
            <input
                type="string"
                class="centered-input"
                placeholder="اسم الشيت"
                value=move || name.get()
                on:input=move |ev| set_name.set(Some(event_target_value(&ev)))
            />
            <div class="date-input-container">
                <label for="start-date">"تاريخ البداية"</label>
                <input
                    type="date"
                    id="start-date"
            value=move || begin.get().map(|x| x.to_string()).unwrap_or_else(|| "".to_string())
                    on:input=move |ev| set_begin.set(event_target_value(&ev).parse().ok())
                />
                <label for="end-date">"تاريخ النهاية"</label>
                <input
                    type="date"
                    id="end-date"
            value=move || end.get().map(|x| x.to_string()).unwrap_or_else(|| "".to_string())
                    on:input=move |ev| set_end.set(event_target_value(&ev).parse().ok())
                />
            </div>
	    <Show
		when=move || offset.get() != 0
		fallback=|_| view!{cx,<></>}
	    >
		<button
		on:click=move |_| set_offset.update(|x| *x -= 5)
		class="btn"
		>
		<span class="up-arrow">"↑"</span>
		</button>
	    </Show>
            <br/>
            <br/>
            <For
                each=move || bills.read(cx).unwrap_or_default()
                key=|s| s.id
                view=move |cx, s| {
                    view! { cx,
                        <A class="button" href=format!("show/{}", s.id)>
                            {s.the_name}
                        </A>
                    }
                }
            />
	    <Show
		when=move || {bills.read(cx).unwrap_or_default().len() >= 5}
		fallback=|_| view!{cx,<></>}
	    >
		<button
		    on:click=move |_| set_offset.update(|x| *x += 5)
		    class="btn"
		>
		    <span class="down-arrow">"↓"</span>
		</button>
	    </Show>
        </section>
    }
}

#[component]
pub fn SheetHead(cx: Scope) -> impl IntoView {
    view! { cx,
        <thead>
            <tr>
                <th>"رقم الفاتورة"</th>
                <th>"التاريخ"</th>
                <th>"رقم التسجيل الضريبي"</th>
                <th>"اسم العميل"</th>
                <th>"تبع"</th>
                <th>"القيمة"</th>
                <th>"ض.ق.م"</th>
                <th>"الخصم"</th>
                <th>"الاجمالي"</th>
            </tr>
        </thead>
    }
}

#[component]
pub fn ShowSheet(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || {
        if let Some(id) = params.with(|params| params.get("id").cloned()) {
            match Uuid::from_str(&id) {
                Ok(id) => IdArg { id },
                Err(_) => IdArg { id: Uuid::nil() },
            }
        } else {
            IdArg { id: Uuid::nil() }
        }
    };

    let bills = create_resource(cx, id, |value| async move {
        invoke::<_, Vec<SellBill>>("get_sheet_sellbills", &value)
            .await
            .unwrap_or_default()
    });

    view! { cx,
        <section>
            <A class="left-corner" href="/sales">
                "->"
            </A>
            <br/>
            <br/>
            <br/>
            <table class="table-excel">
                <SheetHead/>
                <tbody>
                    <For
                        each=move || bills.read(cx).unwrap_or_default()
                        key=|s| s.bill_id
                        view=move |cx, s| {
                            view! { cx, <ShowRow sellbill=s/> }
                        }
                    />
                </tbody>
            </table>
            <Outlet/>
        </section>
    }
}

#[component]
fn ShowRow(cx: Scope, sellbill: SellBill) -> impl IntoView {
    let tax = sellbill
        .total_cost
        .clone()
        .unwrap_or_default()
        .to_f64()
        .unwrap_or_default()
        * 0.14;
    let total = sellbill
        .total_cost
        .clone()
        .unwrap_or_default()
        .to_f64()
        .unwrap_or_default()
        + tax.clone()
        - sellbill.discount.clone().to_f64().unwrap_or_default();

    let bill = create_resource(
        cx,
        || (),
        move |_| async move {
            invoke::<_, Bill>(
                "get_bill",
                &IdArg {
                    id: sellbill.bill_id,
                },
            )
            .await
            .unwrap_or_default()
        },
    );

    let com = create_resource(
        cx,
        || (),
        move |_| async move {
            invoke::<_, Company>(
                "get_company",
                &IdArg {
                    id: sellbill.company_id.unwrap_or_default(),
                },
            )
            .await
            .unwrap_or_default()
        },
    );

    let clt = create_resource(
        cx,
        || (),
        move |_| async move {
            invoke::<_, Client>(
                "get_client",
                &IdArg {
                    id: sellbill.client_id.unwrap_or_default(),
                },
            )
            .await
            .unwrap_or_default()
        },
    );

    view! { cx,
        <tr>
            <td>{move || bill.read(cx).unwrap_or_default().bill_number}</td>
            <td>{move || bill.read(cx).unwrap_or_default().the_date.to_string()}</td>
            <td>{sellbill.tax_number}</td>
            <td>{move || com.read(cx).unwrap_or_default().the_name}</td>
            <td>{move || clt.read(cx).unwrap_or_default().the_name}</td>
            <td>{sellbill.total_cost.map(|x| x.to_string())}</td>
            <td>{format!("{:.2}", tax)}</td>
            <td>{sellbill.discount.to_string()}</td>
            <td>{total.to_string()}</td>
        </tr>
    }
}

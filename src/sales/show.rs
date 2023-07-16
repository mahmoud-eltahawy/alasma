use leptos::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};
use tauri_sys::tauri::invoke;

use bigdecimal::ToPrimitive;
use std::str::FromStr;

use uuid::Uuid;

use crate::shared::alert;

use super::{IdArg,SheetHead};
use models::backend_api::{SellBill,Bill,Company,Client};

#[derive(Serialize,Deserialize)]
struct WriteArgs{
    sheetid : Uuid,
    sellbills : Vec<SellBill>,
}

#[component]
pub fn ShowSheet(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let sheet_id = move || Uuid::from_str(params.
	with(|params| params.get("id").cloned())
	.unwrap_or_default().as_str())
        .unwrap_or_default();

    let bills = create_resource(
	cx,
	|| (),
	move |_| async move {
        invoke::<_, Vec<SellBill>>("get_sheet_sellbills", &IdArg{ id :sheet_id()})
            .await
            .unwrap_or_default()
    });

    let export = move|_| spawn_local(async move {
	    match invoke::<_,()>("write_sells_excel", &WriteArgs{
		sheetid : sheet_id(),
		sellbills: bills.read(cx).unwrap_or_default()
	    }).await {
		Ok(_) => alert("تم بنجاج"),
		Err(err) => alert(err.to_string().as_str())
	    };
	}
    );

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
	    <button on:click=export >"تصدير"</button>
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
        + tax
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

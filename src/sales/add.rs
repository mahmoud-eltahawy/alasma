use super::{Row,CompleteSection,SheetHead,InputRow,InputVariables,NewTd};


use leptos::*;
use leptos_router::*;

use tauri_sys::tauri::invoke;

use models::backend_api::{Bill, Client, Company,NaiveSellBill};
use serde::{Deserialize, Serialize};

use crate::shared::{function::save_sell_bills_to_sheet,alert, new_id};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct SheetArgs {
    id: Uuid,
    name: String,
}

async fn save_new_sheet_header(
    name : String,
) -> Result<Uuid,Box<dyn std::error::Error>>{
    if name.is_empty() {
	return Err("empty name".to_string().into());
    }
    let sheet_id = new_id().await;
    invoke::<SheetArgs, ()>(
	"save_sell_sheet",
	&SheetArgs {
	    id: sheet_id,
	    name,
	},
    )
    .await?;
    Ok(sheet_id)
}


async fn save_sheet_to_db(
    sheet_name: String,
    bill_list: Vec<NaiveSellBill>,
) {
    if sheet_name.is_empty() || bill_list.is_empty() {
        return;
    }
    let sheet_id = match save_new_sheet_header(sheet_name).await {
	Ok(id) => id,
	Err(err) => {
	    alert(format!("Error 3 : {err:?}").as_str());
	    return;
	}
    };
    save_sell_bills_to_sheet(sheet_id, bill_list).await;
}

#[component]
pub fn SaleSheetAdd(cx: Scope) -> impl IntoView {
    let (sheet_name, set_sheet_name) = create_signal(cx, String::from(""));
    let (list, set_list) = create_signal(cx, Vec::<NaiveSellBill>::new());

    let (company_id, set_company_id) = create_signal(cx, None::<Uuid>);
    let (client_id, set_client_id) = create_signal(cx, None::<Uuid>);
    let (company_name, set_company_name) = create_signal(cx, String::new());
    let (client_name, set_client_name) = create_signal(cx, String::new());

    let append = move |InputVariables {
	bill_number,
	tax_number,
	bill_date,
	value,
	discount
    }| {
	let appendable =list.get().iter()
            .all(|y| y.bill.bill_number != bill_number.get() as i64);

        if !appendable {
            return;
        }
        let company = match company_id.get() {
            None => Company {
                id: Uuid::nil(),
                the_name: company_name.get().trim().to_string(),
            },
            Some(id) => Company {
                id,
                the_name: company_name.get(),
            },
        };

        let client = match client_id.get() {
            None => Some(Client {
                id: Uuid::nil(),
                the_name: client_name.get().trim().to_string(),
            }),
            Some(id) => Some(Client {
                id,
                the_name: client_name.get(),
            }),
        };
        let value = value.get();
        let discount = discount.get();
        let tax_number = tax_number.get();
        let bill_number = bill_number.get().try_into().unwrap();
        let the_date = bill_date.get();
        let is_sell = true;

        spawn_local(async move {
            let x = NaiveSellBill {
                bill: Bill {
                    id: new_id().await,
                    bill_number,
                    the_date,
                    is_sell,
                },
                tax_number,
                company,
                client,
                value,
                discount,
            };
	    set_list.update(|xs| xs.push(x));
        });
    };

    let save_sheet = move |_| {
	spawn_local(async move {
	    save_sheet_to_db(sheet_name.get(), list.get()).await;
	    set_list.set(vec![]);
	})
    };

    view! { cx,
        <div>
            <A class="left-corner" href="/sales">
                "->"
            </A>
            <br/>
            <input
                type="string"
                class="centered-input"
                placeholder="اسم الشيت"
                value=move || sheet_name.get()
                on:input=move |ev| set_sheet_name.set(event_target_value(&ev))
            />
            <br/>
            <table class="table-excel">
                <SheetHead/>
                <tbody>
                    <For
                        each=move || list.get()
                        key=|b| b.bill.bill_number
                        view=move |cx, b| {
                            view! { cx,
                                <Row element=b.clone()>
                                    <NewTd set_list=set_list bill_number=b.bill.bill_number/>
                                </Row>
                            }
                        }
                    />
                    <InputRow
                        append=append
                        company_name=company_name
                        set_company_name=set_company_name
                        client_name=client_name
                        set_client_name=set_client_name
                        set_company_id=set_company_id
                        set_client_id=set_client_id
                    />
                </tbody>
            </table>
            <br/>
            <button on:click=save_sheet class="centered-button">
                "حفظ الشيت"
            </button>
            <CompleteSection
                client_name=client_name
                set_client_name=set_client_name
                set_client_id=set_client_id
                company_name=company_name
                set_company_name=set_company_name
                set_company_id=set_company_id
            />
            <Outlet/>
        </div>
    }
}

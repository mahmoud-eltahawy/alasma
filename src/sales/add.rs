use super::{Row,CompleteSection,SheetHead,InputRow,InputVariables,NameArg,NewTd};

use std::collections::HashMap;

use bigdecimal::{BigDecimal, FromPrimitive};
use leptos::*;
use leptos_router::*;

use tauri_sys::tauri::invoke;

use models::backend_api::{Bill, Client, Company, SellBill,NaiveSellBill};
use serde::{Deserialize, Serialize};

use crate::shared::{alert, new_id};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct SheetArgs {
    id: Uuid,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct BillArgs {
    bill: Bill,
}

#[derive(Serialize, Deserialize)]
struct SellBillArgs {
    sellbill: SellBill,
}

#[derive(Serialize, Deserialize)]
struct CompanyArgs {
    company: Company,
}

#[derive(Serialize, Deserialize)]
struct ClientArgs {
    client: Client,
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

async fn get_id_by_name(
    cmd : &str,
    name : String,
) -> Result<Uuid,Box<dyn std::error::Error>>{
    if name.is_empty() {
	return Err("empty name".to_string().into());
    }
    let id = invoke::<NameArg, Uuid>(
	cmd,
	&NameArg {
	    name,
	},
    ).await?;
    Ok(id)
}

async fn get_company_id(
    name : String,
) -> Result<Uuid,Box<dyn std::error::Error>>{
    let id = get_id_by_name("get_company_id", name).await?;
    Ok(id)
}

async fn get_client_id(
    name : String,
) -> Result<Uuid,Box<dyn std::error::Error>>{
    let id = get_id_by_name("get_client_id", name).await?;
    Ok(id)
}

async fn save_company(
    the_name : String,
) -> Result<Company,Box<dyn std::error::Error>>{
    let company = Company {
	id: new_id().await,
	the_name,
    };
    invoke::<CompanyArgs, ()>(
	"save_company",
	&CompanyArgs {
	    company: company.clone(),
	},
    )
    .await?;
    Ok(company)
}

async fn save_client(
    the_name : String,
) -> Result<Client,Box<dyn std::error::Error>>{
    let client = Client {
	id: new_id().await,
	the_name,
    };
    invoke::<ClientArgs, ()>(
	"save_client",
	&ClientArgs {
	    client: client.clone(),
	},
    )
    .await?;
    Ok(client)
}

async fn save_sell_bills_to_sheet(
    sheet_id : Uuid,
    bill_list : Vec<NaiveSellBill>,
){
    let mut saved_companies = HashMap::<String, Uuid>::new();
    let mut saved_clients = HashMap::<String, Uuid>::new();
    for x in bill_list.into_iter() {
	if let Err(err) = invoke::<BillArgs, ()>(
	    "save_bill",
	    &BillArgs {
		bill: x.bill.clone(),
	    },
	)
        .await {
	    alert(format!("Error 1 : {err:?}").as_str());
	    continue;
	};

	let company_id = if !x.company.id.is_nil() {
	    x.company.id
	} else if let Ok(id) = get_company_id(x.company.the_name.clone()).await {
	    id
	} else if let Some(id) = saved_companies.get(&x.company.the_name) {
	    *id
	} else {
	    match save_company(x.company.the_name.clone()).await {
		Ok(Company { id, the_name }) =>{
		    saved_companies.insert(the_name, id);
		    id
		},
		Err(err) => {
		    alert(format!("حفظ {} فشل 5", x.company.the_name).as_str());
		    log!("{err}");
		    continue;
		}
	    }
	};

	let client_id = match &x.client {
	    Some(client) => {
		if !client.id.is_nil() {
		    x.client.map(|x| x.id)
		} else if let Ok(id) = get_client_id(client.the_name.clone()).await {
		    Some(id)
		} else if let Some(id) = saved_clients.get(&client.the_name) {
		    Some(*id)
		} else {
		    match save_client(client.the_name.clone()).await {
			Ok(Client { id, the_name }) =>{
			    saved_clients.insert(the_name, id);
			    Some(id)
			},
			Err(err) => {
			    alert(format!("حفظ {} فشل 5", client.the_name).as_str());
			    log!("{err}");
			    continue;
			}
		    }
		}
	    }
	    None => None,
	};

	let sellbill = SellBill {
	    bill_id: x.bill.id,
	    tax_number: Some(x.tax_number as i64),
	    company_id: Some(company_id),
	    client_id,
	    sheet_id,
	    total_cost: BigDecimal::from_f64(x.value),
	    discount: BigDecimal::from_f64(x.discount).unwrap_or_default(),
	};

	match invoke::<SellBillArgs, ()>(
	    "save_sell_bill",
	    &SellBillArgs { sellbill },
	)
	.await
	{
	    Ok(()) => (),
	    Err(err) => alert(format!("Error 2 : {err:?}").as_str()),
	};
    }
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

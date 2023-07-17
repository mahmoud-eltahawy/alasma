use super::SheetHead;

use std::collections::HashMap;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Local, NaiveDate};
use leptos::*;
use leptos_router::*;

use tauri_sys::tauri::invoke;

use models::backend_api::{Bill, Client, Company, Name, SellBill};
use serde::{Deserialize, Serialize};

use crate::shared::{alert, new_id};
use uuid::Uuid;

#[derive(Clone, Default)]
struct NaiveSellBill {
    bill: Bill,
    tax_number: u64,
    company: Company,
    client: Option<Client>,
    value: f64,
    discount: f64,
}

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

#[derive(Serialize, Deserialize)]
struct NameArg {
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
                            view! { cx, <ShowRow element=b set_list=set_list/> }
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

#[component]
fn CompleteSection(
    cx: Scope,
    client_name: ReadSignal<String>,
    set_client_name: WriteSignal<String>,
    set_client_id: WriteSignal<Option<Uuid>>,
    company_name: ReadSignal<String>,
    set_company_name: WriteSignal<String>,
    set_company_id: WriteSignal<Option<Uuid>>,
) -> impl IntoView {
    view! { cx,
        <>
            <section>
                <Complete
                    name=company_name
                    set_name=set_company_name
                    set_id=set_company_id
                    search_topic=String::from("top_5_companies")
                />
                <Complete
                    name=client_name
                    set_name=set_client_name
                    set_id=set_client_id
                    search_topic=String::from("top_5_clients")
                />
            </section>
        </>
    }
}

fn search(name: String, topic: String, set_result: WriteSignal<Vec<Name>>) {
    spawn_local(async move {
        match invoke::<_, Vec<Name>>(&topic, &NameArg { name }).await {
            Ok(v) => set_result.set(v),
            Err(err) => log!("{:#?}", err),
        };
    });
}

#[component]
fn Complete(
    cx: Scope,
    name: ReadSignal<String>,
    set_name: WriteSignal<String>,
    set_id: WriteSignal<Option<Uuid>>,
    search_topic: String,
) -> impl IntoView {
    let (result, set_result) = create_signal(
        cx,
        vec![],
    );

    create_effect(cx, move |_| {
        let name = name.get();
        if name.is_empty() {
            return;
        }
        search(name, search_topic.clone(), set_result);
    });

    view! { cx,
        <ol>
            <For
                each=move || result.get()
                key=|x| x.id
                view=move |cx, x| {
                    let name = x.the_name.clone();
                    view! { cx,
                        <li on:click=move |_| {
                            if !x.id.is_nil() {
                                set_id.set(Some(x.id));
                                set_name.set(name.clone());
                            }
                        }>
                            <p>{x.the_name}</p>
                        </li>
                    }
                }
            />
        </ol>
    }
}

#[component]
fn ShowRow(
    cx: Scope,
    element: NaiveSellBill,
    set_list: WriteSignal<Vec<NaiveSellBill>>,
) -> impl IntoView {
    let (hover, set_hover) = create_signal(cx, false);
    let tax = element.value * 0.14;
    let total = element.value + tax - element.discount;

    let remove_from_list = move |_| {
        set_list.update(|xs| {
            xs.retain(|x| x.bill.bill_number != element.bill.bill_number)
        })
    };

    view! { cx,
        <tr>
            <td on:mouseleave=move |_| set_hover.set(false)>
                {move || {
                    if hover.get() {
                        view! { cx, <button on:click=remove_from_list>"حذف"</button> }
                    } else {
                        view! { cx, <button on:click=move |_| set_hover.set(true)>{element.bill.bill_number}</button> }
                    }
                }}
            </td>
            <td>{element.bill.the_date.to_string()}</td>
            <td>{element.tax_number}</td>
            <td>{element.company.the_name}</td>
            <td>{element.client.map(|x| x.the_name)}</td>
            <td>{element.value}</td>
            <td>{format!("{:.2}", tax)}</td>
            <td>{element.discount}</td>
            <td>{total}</td>
        </tr>
    }
}

struct InputVariables{
    bill_number : ReadSignal<u64>,
    tax_number : ReadSignal<u64>,
    bill_date : ReadSignal<NaiveDate>,
    value : ReadSignal<f64>,
    discount : ReadSignal<f64>,
}

#[component]
fn InputRow<F>(
    cx: Scope,
    append : F,
    client_name: ReadSignal<String>,
    set_client_name: WriteSignal<String>,
    company_name: ReadSignal<String>,
    set_company_name: WriteSignal<String>,
    set_client_id: WriteSignal<Option<Uuid>>,
    set_company_id: WriteSignal<Option<Uuid>>,
) -> impl IntoView
    where F : Fn(InputVariables) + 'static
{
    let today = Local::now().date_naive();

    let (bill_number, set_bill_number): (ReadSignal<u64>, WriteSignal<u64>) =
        create_signal(cx, 0);
    let (bill_date, set_bill_date) = create_signal(cx, today);
    let (tax_number, set_tax_number): (ReadSignal<u64>, WriteSignal<u64>) =
        create_signal(cx, 0);
    let (value, set_value) = create_signal(cx, 0.0);
    let (discount, set_discount) = create_signal(cx, 0.0);

    let tax = move || value.get() * 0.14;

    let total = move || value.get() + tax() - discount.get();

    let on_click = move |_| append(InputVariables {
	bill_number,
	tax_number,
	bill_date,
	value,
	discount
    }); 

    view! { cx,
        <>
            <tr>
                <td>
                    <input
                        type="number"
                        value=move || bill_number.get()
                        on:input=move |ev| set_bill_number.set(event_target_value(&ev).parse().unwrap_or_default())
                    />
                </td>
                <td>
                    <input
                        type="date"
                        value=move || bill_date.get().to_string()
                        on:input=move |ev| set_bill_date.set(event_target_value(&ev).parse().unwrap_or_default())
                    />
                </td>
                <td>
                    <input
                        type="number"
                        value=move || tax_number.get()
                        on:input=move |ev| set_tax_number.set(event_target_value(&ev).parse().unwrap_or_default())
                    />
                </td>
                <CompletableTd name=company_name set_name=set_company_name set_id=set_company_id/>
                <CompletableTd name=client_name set_name=set_client_name set_id=set_client_id/>
                <td>
                    <input
                        type="number"
                        value=move || value.get()
                        on:input=move |ev| set_value.set(event_target_value(&ev).parse().unwrap_or_default())
                    />
                </td>
                <td>{move || format!("{:.2}", tax())}</td>
                <td>
                    <input
                        type="number"
                        value=move || discount.get()
                        on:input=move |ev| set_discount.set(event_target_value(&ev).parse().unwrap_or_default())
                    />
                </td>
                <td>{move || format!("{:.2}", total())}</td>
            </tr>
            <tr class="spanA">
                <td>
                    <button on:click=on_click>"اضافة"</button>
                </td>
            </tr>
        </>
    }
}

#[component]
fn CompletableTd(
    cx: Scope,
    name: ReadSignal<String>,
    set_name: WriteSignal<String>,
    set_id: WriteSignal<Option<Uuid>>,
) -> impl IntoView {
    view! { cx,
        <td>
            <input
                type="string"
                value=move || name.get()
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                    set_id.set(None);
                }
            />
            <p>{move || name.get()}</p>
        </td>
    }
}

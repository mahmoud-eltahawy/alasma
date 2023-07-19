use leptos::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};
use tauri_sys::tauri::invoke;

use bigdecimal::ToPrimitive;
use std::str::FromStr;

use uuid::Uuid;

use crate::shared::{alert,new_id,component::ProgressBar, function::save_sell_bills_to_sheet};

use super::{Row,NewTd,CompleteSection,IdArg,SheetHead,NaiveSellBill,InputRow,InputVariables};
use models::backend_api::{SellBill,Bill,Company,Client, Sheet, Name};

#[derive(Serialize,Deserialize)]
struct WriteArgs{
    sheetid : Uuid,
    sellbills : Vec<NaiveSellBill>,
}

#[derive(Serialize,Deserialize)]
struct UpdateSheetArgs{
    name : Name
}

async fn update_sheet_name(
    id: Uuid,
    the_name: String,
) -> Result<(),Box<dyn std::error::Error>>{
    if the_name.is_empty() {
	return Ok(());
    }
    invoke::<UpdateSheetArgs, ()>(
	"update_sheet_name",
	&UpdateSheetArgs { name: Name { id, the_name } },
    )
    .await?;
    Ok(())
}

async fn delete_sellbill(
    id: Uuid,
) -> Result<(),Box<dyn std::error::Error>>{
    invoke::<IdArg, ()>(
	"delete_sellbill",
	&IdArg{id},
    )
    .await?;
    Ok(())
}

#[component]
pub fn ShowSheet(cx: Scope) -> impl IntoView {
    let (sheet_name, set_sheet_name) = create_signal(cx, String::from(""));
    let (list, set_list) = create_signal(cx, Vec::<NaiveSellBill>::new());
    let (removed_list, set_removed_list) = create_signal(cx, Vec::<Uuid>::new());

    let (company_id, set_company_id) = create_signal(cx, None::<Uuid>);
    let (client_id, set_client_id) = create_signal(cx, None::<Uuid>);
    let (company_name, set_company_name) = create_signal(cx, String::new());
    let (client_name, set_client_name) = create_signal(cx, String::new());

    let (edit_mode,set_edit_mode) = create_signal(cx, false);

    let (bills_progress,set_bills_progress) = create_signal(cx, (1,1));

    let params = use_params_map(cx);
    let sheet_id = move || Uuid::from_str(params.
	with(|params| params.get("id").cloned())
	.unwrap_or_default().as_str())
        .unwrap_or_default();

    let sheet = create_resource(
	cx,
	|| (),
	move |_| async move {
        invoke::<_, Sheet>("get_sheet", &IdArg{ id :sheet_id()})
            .await
            .unwrap_or_default()
    });

    let bills = create_resource(
	cx,
	|| (),
	move |_| async move {
	    set_bills_progress.set((1,2));
            let sellbills =invoke::<_, Vec<SellBill>>(
		"get_sheet_sellbills",
		&IdArg{ id :sheet_id()}
	    )
            .await
            .unwrap_or_default();
	    set_bills_progress.set((1,sellbills.len() as i32 + 1));
	    let mut list = Vec::new();
	    for sb in sellbills {
		list.push(
		NaiveSellBill{
		    bill : get_bill(sb.bill_id).await,
		    tax_number: sb.tax_number.unwrap_or_default() as u64,
		    company: get_company(sb.company_id.unwrap_or_default()).await,
		    client: get_client(sb.client_id.unwrap_or_default()).await,
		    value: sb.total_cost.unwrap_or_default().to_f64().unwrap_or_default(),
		    discount: sb.discount.to_f64().unwrap_or_default(),
		});
		set_bills_progress.update(|(p,_)| {
		    *p+=1
		})
	    };
	    list
    });

    let append = move |InputVariables {
	bill_number,
	tax_number,
	bill_date,
	value,
	discount
    }| {
	let appendable =list.get().iter()
            .all(|y| y.bill.bill_number != bill_number.get() as i64)
	    && bills.read(cx).unwrap_or_default().iter()
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

    let save_changes = move |_| {
	spawn_local(async move{
	    let Sheet { id, the_name, the_date:_, the_type:_ } = sheet.read(cx).unwrap_or_default();
	    let name = sheet_name.get();
	    if name != the_name{
		match update_sheet_name(id, name).await {
		    Ok(_) => sheet.refetch(),
		    Err(err) => alert(err.to_string().as_str())
		};
	    }
	    let deleted = removed_list.get(); 
	    let added = list.get(); 
	    if !deleted.is_empty() || !added.is_empty() {
		for id in deleted.into_iter() {
		    if let Err(err) = delete_sellbill(id).await {
			alert(err.to_string().as_str());
		    };
		}
		save_sell_bills_to_sheet(id, added).await;
		bills.refetch();
		set_removed_list.set(Vec::new());
		set_list.set(Vec::new());
	    }
	});
	set_edit_mode.set(false)
    };

    view! { cx,
        <section>
            <A class="left-corner" href="/sales">
                "->"
            </A>
            <button class="right-corner" on:click=export>
                "🏹"
            </button>
            <br/>
            <Show
                when=move || edit_mode.get()
                fallback=move |cx| {
                    view! { cx,
                        <>
                            <h1>{move || sheet.read(cx).unwrap_or_default().the_name}</h1>
                            <h4>{move || sheet.read(cx).unwrap_or_default().the_date.to_string()}</h4>
                        </>
                    }
                }
            >
                <input
                    type="string"
                    class="centered-input"
                    placeholder=move || sheet.read(cx).unwrap_or_default().the_name
                    value=move || sheet_name.get()
                    on:input=move |ev| set_sheet_name.set(event_target_value(&ev))
                />
            </Show>
            <br/>
	    <ProgressBar progress=bills_progress/>
            <table class="table-excel">
                <SheetHead/>
                <tbody>
                    <For
                        each=move || bills.read(cx).unwrap_or_default()
                        key=|s| s.bill.id
                        view=move |cx, s| {
                            view! { cx,
                                <Row element=s.clone()>
                                    <BillNumberTd
				    removable=edit_mode
				    bill=s.bill
				    removed=removed_list
				    set_removed=set_removed_list/>
                                </Row>
                            }
                        }
                    />
                    <Show
                        when=move || edit_mode.get()
                        fallback=move |_| {
                            view! { cx, <></> }
                        }
                    >
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
                    </Show>
                </tbody>
            </table>
            <Show
                when=move || edit_mode.get()
                fallback=move |cx| {
                    view! { cx, <button on:click=move |_| set_edit_mode.set(true)>"تعديل"</button> }
                }
            >
                <button on:click=save_changes>"تاكيد"</button>
                <CompleteSection
                    client_name=client_name
                    set_client_name=set_client_name
                    set_client_id=set_client_id
                    company_name=company_name
                    set_company_name=set_company_name
                    set_company_id=set_company_id
                />
            </Show>
            <Outlet/>
        </section>
    }
}

async fn get_bill(id : Uuid) -> Bill{
    invoke::<_, Bill>(
	"get_bill",
	&IdArg {id},
    )
    .await
    .unwrap_or_default()
}

async fn get_company(id : Uuid) -> Company{
    invoke::<_, Company>(
	"get_company",
	&IdArg {id},
    )
    .await
    .unwrap_or_default()
}

async fn get_client(id : Uuid) -> Option<Client>{
    invoke::<_, Client>(
	"get_client",
	&IdArg {id},
    )
    .await.ok()
}

#[component]
fn BillNumberTd(
    cx: Scope,
    removable : ReadSignal<bool>,
    removed : ReadSignal<Vec<Uuid>>,
    set_removed : WriteSignal<Vec<Uuid>>,
    bill : Bill,
) -> impl IntoView{
    let (hover, set_hover) = create_signal(cx, false);

    let remove_from_list = move |_| set_removed
	.update(|xs| xs.push(bill.id));

    let remove_from_remove_list = move |_| set_removed
	.update(|xs| xs.retain(|x| *x != bill.id));
    
    view! { cx,
        <td on:mouseleave=move |_| set_hover.set(false)>
            {move || {
		let can_edit = hover.get() && removable.get();
		let is_removed = removed.get().contains(&bill.id);
                if can_edit && !is_removed{
                    view! { cx, <button on:click=remove_from_list>"حذف"</button> }
                } else if can_edit && is_removed{
                    view! { cx, <button on:click=remove_from_remove_list>"الغاء"</button> }
		} else {
                    view! { cx, <button
			    on:click=move |_| set_hover.set(true)>
			    {
				if is_removed {
				    view!{cx,<><del>{bill.bill_number} "X"</del></>}
				} else {
				    view!{cx,<>{bill.bill_number}</>}
				}
			    }
			    </button>
		    }
                }
            }}
        </td>
    }
}

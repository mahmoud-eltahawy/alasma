use chrono::{NaiveDate,Local};
use leptos::*;
use leptos_router::*;

use models::backend_api::{Name, SheetShearchParams, SheetType,NaiveSellBill};
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;
use uuid::Uuid;
use crate::shared::NameArg;

pub mod add;
pub mod show;

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
    let (offset, set_offset) = create_signal(cx, 0_u64);
    let (begin, set_begin) = create_signal(cx, None::<NaiveDate>);
    let (end, set_end) = create_signal(cx, None::<NaiveDate>);
    let (name, set_name) = create_signal(cx, None::<String>);

    let search_args = move || SheetArgs {
	params: SheetShearchParams {
	    offset: offset.get() as i64,
	    sheet_type: SheetType::Sells,
	    name: name.get(),
	    begin: begin.get(),
	    end: end.get(),
	},
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
                fallback=|_| {
                    view! { cx, <></> }
                }
            >
                <button on:click=move |_| set_offset.update(|x| *x -= 5) class="btn">
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
                when=move || { bills.read(cx).unwrap_or_default().len() >= 5 }
                fallback=|_| {
                    view! { cx, <></> }
                }
            >
                <button on:click=move |_| set_offset.update(|x| *x += 5) class="btn">
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
fn CompleteSection(
    cx: Scope,
    client_name: ReadSignal<String>,
    set_client_name: WriteSignal<String>,
    set_client_id: WriteSignal<Option<Uuid>>,
    company_name: ReadSignal<String>,
    set_company_name: WriteSignal<String>,
    set_company_id: WriteSignal<Option<Uuid>>,
) -> impl IntoView {
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


	fn search(name: String, topic: String, set_result: WriteSignal<Vec<Name>>) {
	    spawn_local(async move {
		match invoke::<_, Vec<Name>>(&topic, &NameArg { name }).await {
		    Ok(v) => set_result.set(v),
		    Err(err) => log!("{:#?}", err),
		};
	    });
	}

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

#[component]
fn Row(
    cx: Scope,
    element: NaiveSellBill,
    children : Children,
) -> impl IntoView {
    let tax = element.value * 0.14;
    let total = element.value + tax - element.discount;

    view! { cx,
        <tr>
            {children(cx)} <td>{element.bill.the_date.to_string()}</td>
            <td>{element.tax_number}</td> <td>{element.company.the_name}</td>
            <td>{element.client.map(|x| x.the_name)}</td> <td>{element.value}</td>
            <td>{format!("{:.2}", tax)}</td> <td>{element.discount}</td>
            <td>{format!("{:.2}", total)}</td>
        </tr>
    }
}

#[component]
fn NewTd(
    cx: Scope,
    bill_number : i64,
    set_list : WriteSignal<Vec<NaiveSellBill>>
) -> impl IntoView{
    let (hover, set_hover) = create_signal(cx, false);

    let remove_from_list = move |_| {
        set_list.update(|xs| {
            xs.retain(|x| x.bill.bill_number != bill_number)
        })
    };
    
    view! { cx,
        <td on:mouseleave=move |_| set_hover.set(false)>
            {move || {
                if hover.get() {
                    view! { cx, <button on:click=remove_from_list>"حذف"</button> }
                } else {
                    view! { cx, <button on:click=move |_| set_hover.set(true)>{bill_number}</button> }
                }
            }}
        </td>
    }
}

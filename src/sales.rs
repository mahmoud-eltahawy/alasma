use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Local;
use leptos::*;
use leptos_router::*;

use tauri_sys::tauri::invoke;

use models::backend_api::{Bill, Client, Company, SellBill};
use serde::{Deserialize, Serialize};

use crate::shared::{alert, new_id};
use uuid::Uuid;

#[derive(Clone)]
struct NaiveSellBill {
    bill: Bill,
    tax_number: u64,
    company: Company,
    client: Option<Client>,
    value: f64,
    discount: f64,
}

#[derive(Clone)]
enum Complete {
    Company,
    Client,
    None,
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
    company : Company,
}

#[derive(Serialize, Deserialize)]
struct ClientArgs {
    client : Client,
}

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    let (sheet_name, set_sheet_name) = create_signal(cx, String::from(""));
    let (sheet_id, set_sheet_id) = create_signal(cx, Uuid::nil());
    let (list, set_list) = create_signal(cx, Vec::<NaiveSellBill>::new());

    let (complete, set_complete) = create_signal(cx, Complete::None);

    let (company_id, set_company_id) = create_signal(cx, None::<Uuid>);
    let (client_id, set_client_id) = create_signal(cx, None::<Uuid>);
    let (company_name, set_company_name) = create_signal(cx, String::new());
    let (client_name, set_client_name) = create_signal(cx, String::new());

    spawn_local(async move {
        set_sheet_id.set(new_id().await);
    });

    let save_sheet = move |_| {
        spawn_local(async move {
            let succ = match invoke::<SheetArgs, ()>(
                "save_sell_sheet",
                &SheetArgs {
                    id: sheet_id.get_untracked(),
                    name: sheet_name.get_untracked(),
                },
            )
            .await
            {
                Ok(()) => None,
                Err(err) => Some(err),
            };

	    if let Some(err) = succ {
                 alert(err.to_string().as_str());
		 return;
	    };

            for x in list.get().into_iter() {
                let succ = match invoke::<BillArgs, ()>(
                    "save_bill",
                    &BillArgs {
                        bill: x.bill.clone(),
                    },
                )
                .await
                {
                    Ok(()) => None,
                    Err(err) => Some(err),
                };

		if let Some(err) = succ {
                    alert(err.to_string().as_str());
		    return;
		};

		let company_id = if x.company.id.is_nil() {
		    let id = new_id().await;
		    let company = Company{
			id,
			the_name : x.company.the_name,
		    };
		    invoke::<CompanyArgs, ()>(
			"save_company",
			&CompanyArgs {company},
		    )
		    .await.unwrap();
		    Some(id)
		} else {
		    Some(x.company.id)
		};

		let client_id = if x.client.clone().is_some_and(|x| x.id.is_nil()) {
		    let id = new_id().await;
		    let client = Client{
			id,
			the_name : x.client.unwrap().the_name,
		    };
		    invoke::<ClientArgs, ()>(
			"save_client",
			&ClientArgs {client},
		    )
		    .await.unwrap();
		    Some(id)
		} else if x.client.clone().is_some_and(|x| !x.id.is_nil()) {
		    x.client.clone().map(|x| x.id)
		} else {
		    None
		};

	        let sellbill = SellBill {
		    bill_id: x.bill.id,
		    tax_number: Some(x.tax_number as i64),
		    company_id,
		    client_id,
		    sheet_id: sheet_id.get(),
		    total_cost: BigDecimal::from_f64(x.value),
		    discount: BigDecimal::from_f64(x.discount).unwrap_or_default(),
		};

                match invoke::<SellBillArgs, ()>(
                    "save_sell_bill",
                    &SellBillArgs {sellbill},
                )
                .await
                {
                    Ok(()) => alert("تم حفظ الشيت بنجاح"),
                    Err(err) => alert(err.to_string().as_str()),
                };
            }
        });
    };

    view! {cx,
       <div>
     <A class="button" href="/">"الرئيسية"</A><br/>
     <input
       type="string"
       style="width : 100%;"
       placeholder="اسم الشيت"
       value=move|| sheet_name.get()
       on:input=move|ev| set_sheet_name.set(event_target_value(&ev))
      /><br/>
      <table class="table-excel">
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
        <tbody>
          <For
            each=move|| list.get()
            key=|b| b.bill.bill_number
        view=move |cx,b| {
            view! {cx,
                <ShowRow element=b set_list=set_list/>
            }
        }
          />
     <InputRow
       set_list=set_list
       list=list
       set_complete=set_complete
       company_name=company_name
       set_company_name=set_company_name
       client_name=client_name
       set_client_name=set_client_name
       company_id=company_id
       set_company_id=set_company_id
       client_id=client_id
       set_client_id=set_client_id
     />
      </tbody>
      </table><br/>
      <ChoicesLine
       complete=complete
       company_name=company_name
       client_name=client_name
       set_company_name=set_company_name
       set_client_name=set_client_name
       set_company_id=set_company_id
       set_client_id=set_client_id
      />
      <button
        on:click=save_sheet
        style="width : 100%;"
      >"حفظ الشيت"</button>
      <Outlet/>
    </div>
    }
}

#[derive(Serialize, Deserialize)]
struct Name {
    name: String,
}

#[component]
fn ChoicesLine(
    cx: Scope,
    complete: ReadSignal<Complete>,
    client_name: ReadSignal<String>,
    company_name: ReadSignal<String>,
    set_client_name: WriteSignal<String>,
    set_company_name: WriteSignal<String>,
    set_client_id: WriteSignal<Option<Uuid>>,
    set_company_id: WriteSignal<Option<Uuid>>,
) -> impl IntoView {
    let (result, set_result) = create_signal(
        cx,
        vec![Company {
            id: Uuid::nil(),
            the_name: String::from("صباح الخير او مساء الخير علي حسب التوقيت"),
        }],
    );

    create_effect(cx, move |_| {
        let cond = complete.get();
        let name = match cond {
            Complete::Company => company_name.get(),
            Complete::Client => client_name.get(),
            Complete::None => String::from(""),
        };

        spawn_local(async move {
            match cond {
                Complete::Company => set_result.set(
                    invoke::<_, Vec<Company>>(
                        "top_5_companies",
                        &Name { name },
                    )
                    .await
                    .unwrap(),
                ),
                Complete::Client => set_result.set(
                    invoke::<_, Vec<Company>>("top_5_clients", &Name { name })
                        .await
                        .unwrap(),
                ),
                Complete::None => (),
            };
        });
    });

    view! {cx,
    <ul>
    <For
        each= move|| result.get()
        key=|x| x.id
	view=move |cx, x| {
	    let name = x.the_name.clone();
	    view! { cx,
		<li on:click=move |_| match complete.get() {
			Complete::Company => {
			    set_company_id.set(Some(x.id));
			    set_company_name.set(name.clone());
			},
			Complete::Client => {
			    set_client_id.set(Some(x.id));
			    set_client_name.set(name.clone());
			},
			Complete::None => (),
		    }><p>{x.the_name}</p></li>
	    }
        }
    />
      </ul>
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

    view! {cx,
    <tr>
      <td
       on:mouseleave=move |_| set_hover.set(false)
       >{move || if hover.get() {
           view! {cx,<button on:click=remove_from_list>"حذف"</button>}
       } else {
           view! {cx,<button
                on:click=move |_| set_hover.set(true)
              >{element.bill.bill_number}</button>
           }
       }
       }</td>
      <td>{element.bill.the_date.map(|x| x.to_string())}</td>
      <td>{element.tax_number}</td>
      <td>{element.company.the_name}</td>
      <td>{element.client.map(|x| x.the_name)}</td>
      <td>{element.value}</td>
      <td>{format!("{:.2}",tax)}</td>
      <td>{element.discount}</td>
      <td>{total}</td>
    </tr>
    }
}

#[component]
fn InputRow(
    cx: Scope,
    set_list: WriteSignal<Vec<NaiveSellBill>>,
    list: ReadSignal<Vec<NaiveSellBill>>,
    client_name: ReadSignal<String>,
    set_client_name: WriteSignal<String>,
    company_name: ReadSignal<String>,
    set_company_name: WriteSignal<String>,
    client_id: ReadSignal<Option<Uuid>>,
    set_client_id: WriteSignal<Option<Uuid>>,
    company_id: ReadSignal<Option<Uuid>>,
    set_company_id: WriteSignal<Option<Uuid>>,
    set_complete: WriteSignal<Complete>,
) -> impl IntoView {
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

    let appendable = move || {
        list.get()
            .into_iter()
            .all(|y| y.bill.bill_number != bill_number.get() as i64)
    };

    let on_click = move |_| {
        if !appendable() {
            return;
        }
        spawn_local(async move {
            let x = NaiveSellBill {
                bill: Bill {
                    id: new_id().await,
                    bill_number: bill_number.get().try_into().unwrap(),
                    the_date: Some(bill_date.get()),
                    is_sell: true,
                },
                tax_number: tax_number.get(),
                company: match company_id.get() {
                    None => Company {
                        id: Uuid::nil(),
                        the_name: company_name.get(),
                    },
                    Some(id) => Company {
                        id,
                        the_name: company_name.get(),
                    },
                },
                client: match client_id.get() {
                    None => Some(Client {
                        id: Uuid::nil(),
                        the_name: client_name.get(),
                    }),
                    Some(id) => Some(Client {
                        id,
                        the_name: client_name.get(),
                    }),
                },
                value: value.get(),
                discount: discount.get(),
            };
            set_list.update(|xs| xs.push(x))
        });
    };

    view! {cx,
       <>
       <tr>
     <td><input
       type="number"
       value=move|| bill_number.get()
       on:input=move|ev| set_bill_number.set(event_target_value(&ev).parse().unwrap())
     /></td>
     <td><input
       type="date"
       value=move|| bill_date.get().to_string()
       on:input=move|ev| set_bill_date.set(event_target_value(&ev).parse().unwrap())
     /></td>
     <td><input
       type="number"
       value=move|| tax_number.get()
       on:input=move|ev| set_tax_number.set(event_target_value(&ev).parse().unwrap())
     /></td>
     <CompletableTd
	   name=company_name
	   set_name=set_company_name
	   set_id=set_company_id
	   on_input_do=move || set_complete.set(Complete::Company)/>
     <CompletableTd
	   name=client_name
	   set_name=set_client_name
	   set_id=set_client_id
	   on_input_do=move || set_complete.set(Complete::Client)/>
     <td><input
       type="number"
       value=move|| value.get()
       on:input=move|ev| set_value.set(event_target_value(&ev).parse().unwrap())
     /></td>
     <td>{move|| format!("{:.2}",tax())}</td>
     <td><input
       type="number"
       value=move || discount.get()
       on:input=move|ev| set_discount.set(event_target_value(&ev).parse().unwrap())
     /></td>
         <td>{move|| format!("{:.2}",total())}</td>
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
fn CompletableTd<U>(
    cx: Scope,
    name : ReadSignal<String>,
    set_name : WriteSignal<String>,
    set_id : WriteSignal<Option<Uuid>>,
    on_input_do : U,
) -> impl IntoView
where
    U : Fn() -> () + 'static,
{
    
    view! {cx,
     <td><input
       type="string"
       value=move|| name.get()
       on:input=move|ev| {
	 on_input_do();
         set_name.set(event_target_value(&ev));
	 set_id.set(None);
       }/>
       <p>{move|| name.get()}</p>
     </td>
    }
}

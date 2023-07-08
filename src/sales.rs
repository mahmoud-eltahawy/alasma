use std::time::Duration;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDate, Local};
use leptos::*;
use leptos_router::*;

use tauri_sys::tauri::invoke;

use models::backend_api::{Sheet,SheetType, SellBill, Bill, Company, Client};
use serde::{Serialize, Deserialize};

use uuid::Uuid;
use crate::shared::{Non,alert, new_id};

#[derive(Clone)]
struct NaiveSellBill {
    bill : Bill,
    tax_number : u64,
    company : Company,
    client : Option<Client>,
    value : f64,
    discount : f64,
}

#[derive(Serialize,Deserialize)]
struct SheetArgs{
    id : Uuid,
    name: String,
}

#[derive(Serialize,Deserialize)]
struct BillArgs{
    sellbill : SellBill
}

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    let (sheet_name,set_sheet_name) = create_signal(cx,String::from(""));
    let (sheet_id,set_sheet_id) = create_signal(cx,Uuid::nil());
    let (list,set_list) = create_signal(cx, Vec::<NaiveSellBill>::new());

    spawn_local(async move {
	let id = new_id().await;
	set_sheet_id.set(id);
    });

    let to_sell_bill =move |x : NaiveSellBill| SellBill{
	    bill_id : x.bill.id,
	    tax_number : Some(x.tax_number as i64),
	    company_id : Some(x.company.id),
	    client_id : x.client.map(|x| x.id),
	    sheet_id : sheet_id.get(),
	    total_cost : BigDecimal::from_f64(x.value),
	    discount : BigDecimal::from_f64(x.discount).unwrap_or_default(),
	};

    let save_sheet = move |_| {
      log!("begin");
      spawn_local(async move {
	  let succ = match invoke::<SheetArgs,()>("save_sell_sheet", &SheetArgs{
	      id : sheet_id.get_untracked(),
	      name : sheet_name.get_untracked(),
	  }).await {
	      Err(_) => false,
	      Ok(()) => true,
	  };

	  if !succ {
	      return;
	  }

	  for x in list.get().into_iter() {
	    match invoke::<BillArgs,()>("save_sell_bill", &BillArgs{
		 sellbill : to_sell_bill(x) 
	      }).await {
		Ok(()) => alert("تم حفظ الشيت بنجاح"),
		Err(err) => alert(err.to_string().as_str())
	    };
	  }
      });
    };

    view!{cx,
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
		        <ShowRow element={b} writer=set_list/>
		    }
		}
	      />
	  <InputRow writer=set_list list=list />
	    </tbody>
	  </table><br/>
	  <button
	    on:click=save_sheet
	    style="width : 100%;"
	  >"حفظ الشيت"</button>
	  <Outlet/>
	</div>
    }
}

#[component]
fn ShowRow(cx: Scope,element : NaiveSellBill,writer : WriteSignal<Vec<NaiveSellBill>>) -> impl IntoView{
    let (hover,set_hover) = create_signal(cx,false);
    let tax = element.value * 0.14;
    let total =element.value + tax - element.discount;

    let remove_from_list = move |_| {
	writer.update(|xs|
		      xs.retain(|x| x.bill.bill_number != element.bill.bill_number))
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
    writer : WriteSignal<Vec<NaiveSellBill>>,
    list: ReadSignal<Vec<NaiveSellBill>>
) -> impl IntoView{
    let today = Local::now().date_naive();
    
    let (bill_number,set_bill_number) : (ReadSignal<u64>,WriteSignal<u64>) = create_signal(cx, 0);
    let (bill_date,set_bill_date) = create_signal(cx, today);
    let (tax_number,set_tax_number) : (ReadSignal<u64>,WriteSignal<u64>)= create_signal(cx, 0);
    let (company_id,set_company_id) = create_signal(cx, None::<Uuid>);
    let (client_id,set_client_id) = create_signal(cx, None::<Uuid>);
    let (company_name,set_company_name) = create_signal(cx, String::new());
    let (client_name,set_client_name) = create_signal(cx, String::new());
    let (value,set_value) = create_signal(cx, 0.0);
    let (discount,set_discount) = create_signal(cx, 0.0);

    let tax =move|| value.get() * 0.14;

    let total =move|| value.get() + tax() - discount.get();

    let appendable =move || list.get().into_iter()
	.all(|y| y.bill.bill_number != bill_number.get() as i64);

    let on_click =move |_| {
	if !appendable() {
	    return;
	}
      spawn_local(async move {
	  let x= NaiveSellBill{
	    bill : Bill{
		id : new_id().await,
		bill_number : bill_number.get().try_into().unwrap(),
		the_date : Some(bill_date.get()),
		is_sell : true,
	    },
	    tax_number : tax_number.get(),
	    company : match company_id.get() {
		None => Company{
		    id : new_id().await,
		    the_name : company_name.get(),
		},
		Some(id) => Company{
		    id,
		    the_name : company_name.get(),
		},
	    } ,
	    client :  match client_id.get() {
		None => Some(Client{
		    id : new_id().await,
		    the_name : client_name.get(),
		}),
		Some(id) => Some(Client{
		    id,
		    the_name : client_name.get(),
		}),
	    },
	    value : value.get(),
	    discount : discount.get(),
	  };
	  writer.update(|xs| xs.push(x))
      });
    };

    view!{cx,
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
	<td><input
	  type="string"
	  value=move|| company_name.get()
	  on:input=move|ev| set_company_name.set(event_target_value(&ev))
	/></td>
	<td><input
	  type="string"
	  value=move|| client_name.get()
	  on:input=move|ev| set_client_name.set(event_target_value(&ev))
	/></td>
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
	  <tr class="spanA"><td>
	    <button on:click=on_click>"اضافة"</button>
	  </td>
      </tr>
      </>
    }
}

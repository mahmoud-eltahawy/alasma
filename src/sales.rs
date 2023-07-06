use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;

use models::backend_api::SellBill;

#[derive(Clone)]
struct NaiveSellBill {
    bill_number : u64,
    bill_date : NaiveDate,
    tax_number : u64,
    company_name : String,
    client_name : String,
    value : f64,
    discount : f64,
}

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    let (list,set_list) = create_signal(cx, Vec::<NaiveSellBill>::new());
    let add_to_list = move |x: NaiveSellBill| {
	set_list.update(|xs| xs.push(x))
    };

    create_effect(cx,move |_| {
	log!("{}",list.get().len())
    });

    view!{cx,
       <div>
	  <A class="button" href="/">"الرئيسية"</A><br/>
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
	        key=|b| b.bill_number
		view=move |cx,b| {
		    view! {cx,
		        <ShowRow element={b} writer=set_list/>
		    }
		}
	      />
	      <InputRow add=add_to_list/>
	    </tbody>
	  </table>
	  <Outlet/>
	</div>
    }
}

#[component]
fn ShowRow(cx: Scope,element : NaiveSellBill,writer : WriteSignal<Vec<NaiveSellBill>>) -> impl IntoView{
    let (hover,set_hover) = create_signal(cx,false);
    let x = element.value;
    let y = 14.0 / 100.0; 
    let tax = x * y;
    let total =element.value + tax - element.discount;

    let remove_from_list = move |_| {
	writer.update(|xs| xs.retain(|xs| xs.bill_number != element.bill_number))
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
		      >{element.bill_number}</button>
	       }
	   }
	   }</td>
	  <td>{element.bill_date.to_string()}</td>
	  <td>{element.tax_number}</td>
	  <td>{element.company_name}</td>
	  <td>{element.client_name}</td>
	  <td>{element.value}</td>
	  <td>{tax}</td>
	  <td>{element.discount}</td>
	  <td>{total}</td>
	</tr>
    }
}

#[component]
fn InputRow<T>(cx: Scope,add : T) -> impl IntoView
    where T : Fn(NaiveSellBill) -> () + 'static
{
    let (bill_number,set_bill_number) : (ReadSignal<u64>,WriteSignal<u64>) = create_signal(cx, 0);
    let (bill_date,set_bill_date) = create_signal(cx, NaiveDate::from_ymd_opt(2023, 7, 7).unwrap());
    let (tax_number,set_tax_number) : (ReadSignal<u64>,WriteSignal<u64>)= create_signal(cx, 0);
    let (company_name,set_company_name) = create_signal(cx, String::new());
    let (client_name,set_client_name) = create_signal(cx, String::new());
    let (value,set_value) = create_signal(cx, 0.0);
    let (discount,set_discount) = create_signal(cx, 0.0);

    let tax =move|| {
	let x = value.get();
	let y = 14.0 / 100.0; 
	x * y
    };
    let total =move|| value.get() + tax() - discount.get();

    let on_click =move|_| {
	let sell_bill = NaiveSellBill{
	    bill_number : bill_number.get(),
	    bill_date : bill_date.get(),
	    tax_number : tax_number.get(),
	    company_name : company_name.get(),
	    client_name : client_name.get(),
	    value : value.get(),
	    discount : discount.get(),
	};
	add(sell_bill)
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


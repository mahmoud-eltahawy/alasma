use leptos::*;

use bigdecimal::{BigDecimal, FromPrimitive};
use std::collections::HashMap;
use tauri_sys::tauri::invoke;
use crate::shared::{alert,new_id,NameArg};
use models::backend_api::{SellBill,Client,Company,NaiveSellBill,Bill};
use uuid::Uuid;

use serde::{Serialize,Deserialize};

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

pub async fn save_sell_bills_to_sheet(
    sheet_id : Uuid,
    bill_list : Vec<NaiveSellBill>,
){
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

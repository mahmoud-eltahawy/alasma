use excel_io::*;

fn main() {

    // let (okeys,erros) = sells::map_rows("/home/mahmoud/excel/sells/مبيعات العاصمه.lnk.xlsx", "المبيعات").unwrap().split();
    let (okeys,erros) = boughts::map_rows("/home/mahmoud/excel/bought/مشتريات ديار شهر 6.xlsx", "اصناف ").unwrap().split();

    okeys.into_iter().for_each(|x| println!("{:#?}",x));
    erros.into_iter().take(20).for_each(|x| println!("{:#?}",x));
}

// use chrono::{NaiveDate, Days};
// fn days_to_date(days : f64) -> Option<NaiveDate> {
//     let Some(begin) = NaiveDate::from_ymd_opt(1900, 1, 1) else {
//         return None;
//     };
//     let Some(date) = begin.checked_add_days(Days::new(days as u64)) else {
//         return None;
//     };
//     Some(date)
// }

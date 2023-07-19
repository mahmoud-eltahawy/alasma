use models::backend_api::NaiveSellBill;
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};

use uuid::Uuid;

use std::path::MAIN_SEPARATOR;

use crate::AppState;

pub async fn write_sells(
    app_state: &AppState,
    sheet_id: Uuid,
    sell_bills: Vec<NaiveSellBill>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "التاريخ")?;
    worksheet.write_string(0, 1, "رقم الفاتورة")?;
    worksheet.write_string(0, 2, "رقم التسجيل الضريبي")?;
    worksheet.write_string(0, 3, "اسم العميل")?;
    worksheet.write_string(0, 4, "تبع")?;
    worksheet.write_string(0, 5, "القيمة")?;
    worksheet.write_string(0, 6, "ض.ق.م")?;
    worksheet.write_string(0, 7, "الخصم")?;
    worksheet.write_string(0, 8, "الاجمالي")?;

    let sheet = super::api::find_sheet_by_id(app_state, sheet_id).await?;

    for (index, sell_bill) in sell_bills.into_iter().enumerate() {
        let row = index as u32 + 1;
        let value = sell_bill.value;
        let discount = sell_bill.discount;
        let tax = value * 0.14;
        let total = value + tax - discount;
        let bill = sell_bill.bill;
        let com = sell_bill.company;
        let clt = sell_bill.client.unwrap_or_default();
        worksheet.write_string(row, 0, bill.the_date.to_string())?;
        worksheet.write_number(row, 1, bill.bill_number as f64)?;
        worksheet.write_number(row, 2, sell_bill.tax_number as f64)?;
        worksheet.write_string(row, 3, com.the_name)?;
        worksheet.write_string(row, 4, clt.the_name)?;
        worksheet.write_number(row, 5, value)?;
        worksheet.write_number(row, 6, tax)?;
        worksheet.write_number(row, 7, discount)?;
        worksheet.write_number(row, 8, total)?;
    }

    worksheet.autofit();
    worksheet.set_row_height(0, 25)?;
    worksheet.set_row_format(
        0,
        &Format::new()
            .set_background_color(Color::Orange)
            .set_font_size(14)
            .set_reading_direction(2)
            .set_bold()
            .set_border(FormatBorder::DashDotDot),
    )?;

    worksheet.set_right_to_left(true);

    worksheet.set_name("مبيعات")?;

    let file_path = format!(
        "{}{MAIN_SEPARATOR}Downloads{MAIN_SEPARATOR}.xlsx",
        dirs::home_dir().unwrap_or_default().display()
    );

    let file_name = format!(
        "{}-{}-{}.xlsx",
        "شيت مبيعات باسم", sheet.the_name, sheet.the_date
    );
    let path_name = file_path + &file_name;
    workbook.save(&path_name)?;

    Ok(())
}

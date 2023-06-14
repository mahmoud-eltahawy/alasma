use calamine::DataType;
use super::*;

#[derive(Debug)]
pub enum SellsError{
    Date,
    PillNumber,
    TaxNumber,
    ClientName,
    Value,
}

#[derive(Debug)]
pub struct SellsBill{
    pub days : f64,
    pub pill_number : f64,
    pub tax_number : f64,
    pub client_name : String,
    pub the_value : f64,
}

pub fn map_rows(
    path: &str,
    range_name : &str,
) -> Result<Vec<Seed<SellsBill,SellsError>>,Box<dyn std::error::Error>>{
    return rows_mapper(path, range_name, |(index,row)| {
        let mut errors = Vec::new();
        let mut client_name = &"".to_string();
        let days;
        let pill_number;
        let tax_number;
        let value;

        if let DataType::DateTime(new_days) = &row[0] {
            days = new_days;
        } else {
            days = &0.0;
            errors.push(SellsError::Date);
        };
        if let DataType::Float(new_pill_number) = &row[1] {
            pill_number = new_pill_number;
        } else {
            pill_number = &0.0;
            errors.push(SellsError::PillNumber);
        };
        if let DataType::Float(new_tax_number) = &row[2] {
            tax_number = new_tax_number;
        } else {
            tax_number = &0.0;
            errors.push(SellsError::TaxNumber);
        };
        if let DataType::String(new_client_name) = &row[3] {
            client_name = new_client_name;
        } else {
            errors.push(SellsError::ClientName);
        };
        if let DataType::Float(new_value) = &row[4] {
            value = new_value;
        } else {
            value = &0.0;
            errors.push(SellsError::Value);
        };

        if !errors.is_empty() {
            return (None,Some((errors,index + 1)));
        };

        let bill = SellsBill {
            days : *days,
            pill_number : *pill_number,
            tax_number : *tax_number,
            client_name : client_name.to_string(),
            the_value : *value,
        };

        return (Some(bill),None);
    });
}

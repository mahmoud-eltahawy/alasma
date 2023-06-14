use calamine::DataType;
use super::*;

#[derive(Debug)]
pub enum BoughtsError{
    PillNumber,
    CargoName,
    CargoNumber,
    CargoCost,
    Days,
}

#[derive(Debug)]
pub struct BoughtssBill{
    pub pill_number : f64,
    pub cargo_name : String,
    pub cargo_number : f64,
    pub cargo_cost : f64,
    pub days : f64,
}

pub fn map_rows(
    path: &str,
    range_name : &str,
) -> Result<Vec<Seed<BoughtssBill,BoughtsError>>,Box<dyn std::error::Error>>{
    return rows_mapper(path, range_name, |(index,row)| {
        let mut errors = Vec::new();
        let pill_number;
        let mut cargo_name = &"".to_string();
        let cargo_number;
        let cargo_cost;
        let days;

        if let DataType::Float(new_pill_number) = &row[0] {
            pill_number = new_pill_number;
        } else {
            pill_number = &0.0;
            errors.push(BoughtsError::PillNumber);
        };
        if let DataType::String(new_cargo_name) = &row[1] {
            cargo_name = new_cargo_name;
        } else {
            errors.push(BoughtsError::CargoName);
        };
        if let DataType::Float(new_cargo_number) = &row[2] {
            cargo_number = new_cargo_number;
        } else {
            cargo_number = &0.0;
            errors.push(BoughtsError::CargoNumber);
        };
        if let DataType::Float(new_cargo_cost) = &row[3] {
            cargo_cost = new_cargo_cost;
        } else {
            cargo_cost = &0.0;
            errors.push(BoughtsError::CargoCost);
        };
        if let DataType::DateTime(new_days) = &row[7] {
            days = new_days;
        } else {
            days = &0.0;
            errors.push(BoughtsError::Days);
        };

        if !errors.is_empty() {
            return (None,Some((errors,index + 1)));
        };

        let bill = BoughtssBill {
            pill_number: *pill_number,
            cargo_name: cargo_name.to_owned(),
            cargo_number: *cargo_number,
            cargo_cost: *cargo_cost,
            days: *days
        };

        return (Some(bill),None);
    });
}

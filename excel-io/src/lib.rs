pub mod sells;
pub mod boughts;

use calamine::{open_workbook, Xlsx, Reader, DataType};

pub type Seed<T,E> = (Option<T>,Option<(Vec<E>,usize)>);
pub type SeedSplited<T,E> = (Vec<T>,Vec<(Vec<E>,usize)>);

pub trait Spliter<T,E> {
    fn split(self) -> SeedSplited<T,E>;
}

impl<T,E> Spliter<T,E> for Vec<Seed<T,E>> {
    fn split(self) -> SeedSplited<T,E>{
        let (vt,ve) : (Vec<_>,Vec<_>) = self.into_iter().unzip();
        let vt : Vec<_> = vt.into_iter().flat_map(|x| x).collect();
        let ve : Vec<_> = ve
            .into_iter()
            .flat_map(|x| match x {
                Some(v) if !v.0.is_empty() => Some(v),
                _=> None
            }
        ).collect();

        return (vt,ve);
    }
}

pub fn rows_mapper<F,T,E>(
    path: &str,
    range_name : &str,
    mapper : F,
) -> Result<Vec<Seed<T,E>>,Box<dyn std::error::Error>>
    where F : Fn((usize,&[DataType])) -> Seed<T,E>
{
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = match workbook.worksheet_range(range_name) {
        Some(Ok(range)) => range,
        _ => return Err("Error geting the sheet range".to_string().into())
    };
    let result = range
        .rows()
        .enumerate()
        .map(mapper)
        .collect::<Vec<_>>();

    Ok(result)
}

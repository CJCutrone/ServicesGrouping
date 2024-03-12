use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use crate::model::user::User;

pub fn from_excel(path: &str) -> Result<(), calamine::Error> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook
        .worksheet_range("Users")
        .map_err(|_| calamine::Error::Msg("Cannot find sheet 'Users'"))?;

    let iter_records = RangeDeserializerBuilder::with_headers(&["first_name", "last_name", "groups"])
        .from_range(&range)?;

    for result in iter_records {
        let record: User = result.unwrap();
        println!("{:?}", record);
    }

    Ok(())
}
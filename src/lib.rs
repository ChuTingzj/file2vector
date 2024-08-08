use wasm_bindgen::prelude::*;
use calamine::{Reader, Xlsx, Data};
use std::io::Cursor;

#[wasm_bindgen]
pub fn parse(data:&[u8])->JsValue{
    let cursor = Cursor::new(data);
    let mut workbook = Xlsx::new(cursor).unwrap();
    let sheet_names = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheet_names[0]).unwrap();
    let mut result:Vec<Vec<String>> = Vec::new();
    for row in range.rows(){
        let row_data:Vec<String> = row.iter().map(|cell|{
            let cell = cell.to_owned();
             match cell {
                Data::String(v)=>v,
                Data::Bool(v)=>v.to_string(),
                Data::Int(v)=>v.to_string(),
                Data::Float(v)=>v.to_string(),
                Data::DateTime(v)=>v.to_string(),
                Data::DateTimeIso(v)=>v.to_string(),
                Data::DurationIso(v)=>v.to_string(),
                Data::Error(_)=>panic!("err"),
                Data::Empty=>String::new()
             }
        }).collect();
        result.push(row_data);
    };

    JsValue::from_serde(&result).unwrap()
    
}
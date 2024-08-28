extern crate chrono;

use chrono::*;

pub fn review_date() {
    let date =  Local.with_ymd_and_hms(2010, 10, 21, 5, 19, 09);
    println!("date printed - {:?}", date);
}
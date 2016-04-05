//! Utility functions implementing compression algorithms

const BITS_YEAR: u8 = 5;
const BITS_MONTH: u8 = 4;
const BITS_DAY: u8 = 5;
const BITS_DATE: u8 = BITS_YEAR + BITS_MONTH + BITS_DAY;

const BITS_HOUR: u8 = 5;
const BITS_MINUTE: u8 = 6;

pub fn compress_year(year: u16) -> u8 {
    let result: i16 = (year as i16) - 2016;
    if result < 0 { 0 } else { result as u8 }
}

pub fn compress_date(year: u16, month: u8, day: u8) -> u16 {
    let c_year = compress_year(year);
    let result: u16 = (c_year as u16) | ((month as u16) << BITS_YEAR) | ((day as u16) << (BITS_YEAR + BITS_MONTH));

    result
}

pub fn compress_time(hour: u8, minute: u8) -> u16 {
    let result: u16 = (hour as u16) | ((minute as u16) << BITS_HOUR);

    result
}

pub fn compress_date_time(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> u32 {
    let date = compress_date(year, month, day);
    let time = compress_time(hour, minute);
    let result: u32 = (date as u32) | ((time as u32) << BITS_DATE);

    result
}

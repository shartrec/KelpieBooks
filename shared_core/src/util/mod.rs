/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */
pub mod info;

pub fn format_currency(amount: &i64) -> String {
    let abs_amount = amount.abs();
    let sign = if *amount < 0 { "-" } else { "" };
    let dollars = abs_amount / 100;
    let cents = abs_amount % 100;

    let dollars_str = dollars.to_string();

    format!("{}{}.{:02}", sign, dollars_str, cents)
}

pub fn format_currency_typ(amount: &i64) -> String {
    let formatted = format_currency(amount);
    // In Typst, a hyphen at the start of a line can be interpreted as a list item.
    // Using a "minus" sign U+2212 is safer.
    if formatted.starts_with('-') {
        format!("−{}", &formatted[1..])
    } else {
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(&123456), "1234.56");
        assert_eq!(format_currency(&-123456), "-1234.56");
        assert_eq!(format_currency(&123), "1.23");
        assert_eq!(format_currency(&1), "0.01");
        assert_eq!(format_currency(&0), "0.00");
        assert_eq!(format_currency(&222020), "2220.20");
    }

    #[test]
    fn test_parse() {
        let value = "2220.20";
        if let Ok(amount) = value.parse::<f64>() {
            assert_eq!(amount, 2220.20);
            assert_eq!((amount * 100.0f64).round(), 222020f64);
        }
    }
}

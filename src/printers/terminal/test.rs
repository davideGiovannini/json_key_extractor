mod test_terminal_printer {
    use crate::Case;
    use crate::TerminalPrinter;
    use prettytable::{cell, row};

    use serde_json::Number;

    use crate::printers::CasePrinter;

    use crate::app::ColorOption;

    fn colored_string(string: &str) -> String {
        let mut table = TerminalPrinter::setup_prettytable();

        table.add_row(row![format!("{}", string)]);
        table.to_string()
    }

    #[test]
    fn test_null() {
        let value = Case::Null;
        let mut buffer: Vec<u8> = Vec::new();

        TerminalPrinter
            .write(&mut buffer, &value, ColorOption::Never)
            .unwrap();

        assert_eq!(colored_string("null"), String::from_utf8(buffer).unwrap())
    }

    #[test]
    fn test_single_values() {
        let string_case = Case::from_string("asd");
        let date_case = Case::from_string("2017-03-12");
        let bool_case = Case::from_boolean();
        let int_case = Case::from_number(&(2.into()));
        let float_case = Case::from_number(&(Number::from_f64(2f64).unwrap()));

        let mut printer = TerminalPrinter;

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &string_case, ColorOption::Never)
            .unwrap();
        assert_eq!(colored_string("string"), String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &bool_case, ColorOption::Never)
            .unwrap();
        assert_eq!(colored_string("bool"), String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &date_case, ColorOption::Never)
            .unwrap();
        assert_eq!(colored_string("date"), String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &int_case, ColorOption::Never)
            .unwrap();
        assert_eq!(colored_string("int"), String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &float_case, ColorOption::Never)
            .unwrap();
        assert_eq!(colored_string("float"), String::from_utf8(buffer).unwrap());
    }

    #[test]
    fn test_multiple_values() {
        let string_case = Case::from_string("asd");
        let date_case = Case::from_string("2017-03-12");
        let bool_case = Case::from_boolean();
        let int_case = Case::from_number(&(2.into()));
        let float_case = Case::from_number(&(Number::from_f64(2f64).unwrap()));

        let string_date = string_case.clone() + date_case.clone();
        let int_float_bool = int_case.clone() + float_case.clone() + bool_case.clone();
        let all = string_case.clone()
            + date_case.clone()
            + bool_case.clone()
            + int_case.clone()
            + float_case.clone();

        let all_res = colored_string(&vec!["bool", "date", "float", "int", "string"].join(" | "));
        let int_float_bool_res = colored_string(&vec!["bool", "float", "int"].join(" | "));
        let string_date_res = colored_string(&vec!["date", "string"].join(" | "));

        let mut printer = TerminalPrinter;

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &all, ColorOption::Never)
            .unwrap();
        assert_eq!(all_res, String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &int_float_bool, ColorOption::Never)
            .unwrap();
        assert_eq!(int_float_bool_res, String::from_utf8(buffer).unwrap());

        let mut buffer: Vec<u8> = Vec::new();
        printer
            .write(&mut buffer, &string_date, ColorOption::Never)
            .unwrap();
        assert_eq!(string_date_res, String::from_utf8(buffer).unwrap());
    }
}

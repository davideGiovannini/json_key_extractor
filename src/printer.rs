
use data::Case;

pub fn pretty_print(case: &Case, prefix: &str) -> String {
    use Case::*;

    let mut output = String::new();

    match *case {
        Value(ref val) => return format!("{:#?}", val).to_lowercase(),
        Values(ref vals) => {
            return vals.into_iter()
                       .map(|a| format!("{:?}", a).to_lowercase())
                       .fold("|".to_string(), |a, b| format!("{}{}|", a, b))
                       .to_lowercase()
        }

        Array(ref arr) => {
            if arr.len() == 1 {
                match arr[0] {
                    Case::Value(ref t) => return format!("[{:#?}]", t).to_lowercase(),
                    Case::Object(_) => {
                        return format!("[object]\n{}",
                                       pretty_print(&arr[0], &format!("[{}]", prefix)))
                    }
                    _ => (),
                }
            }
            return format!("{:#?}", arr);
        }
        Object(ref obj) => {
            let mut keys = obj.keys().collect::<Vec<&String>>();
            keys.sort();
            output.push_str("object\n");
            for k in keys {
                if prefix.is_empty() {
                    output.push_str(&format!("{:<60}", k));
                } else {
                    output.push_str(&format!("{:<60}", format!("{}.{}", prefix, k)));
                }
                output.push_str(&format!("{:>20}",
                                        pretty_print(obj.get(k).unwrap(),
                                                     &format!("{}.{}", prefix, k))));
                output.push('\n');
            }
            "".to_string()
        }
        Null => return "<null>".to_string(),
        Multi(ref cases) =>
            return cases.into_iter()
                       .map(print_type)
                       .fold("|".to_string(), |a, b| format!("{}{}|", a, b))
                       .to_lowercase()
    };

    output
}


fn print_type(case: &Case) -> String{
    use Case::*;
    match *case {
        Value(ref val) => return format!("{:#?}", val).to_lowercase(),
        Values(ref vals) => {
            return vals.into_iter()
                       .map(|a| format!("{:?}", a).to_lowercase())
                       .fold("|".to_string(), |a, b| format!("{}{}|", a, b))
                       .to_lowercase()
        }

        Array(ref arr) => {
            if arr.len() == 1 {
                match arr[0] {
                    Case::Value(ref t) => return format!("[{:#?}]", t).to_lowercase(),
                    Case::Object(_) => {
                        return "[object]".to_string()
                    }
                    _ => (),
                }
            }
            return format!("{:#?}", arr);
        }
        _ => "type".to_string()
    }
}

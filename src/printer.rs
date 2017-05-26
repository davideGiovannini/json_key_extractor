
use data::Case;

pub fn pretty_print(case: &Case, prefix: &str) -> String {
    use Case::*;

    let mut output = String::new();

    match *case {
        Values(ref vals) => return format!("{}", vals),

        Array(ref arr) => {
            if arr.len() == 1 {
                match arr[0] {
                    Case::Values(ref t) => return format!("[{}]", t).to_lowercase(),
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
    };
    output
}

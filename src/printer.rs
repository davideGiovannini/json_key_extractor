
use data::Case;
use data::Object;

pub fn pretty_print(case: &Case, prefix: &str) -> String {
    use Case::*;

    match *case {
        Values(ref vals) => format!("{}", vals),

        Array(ref arr) => {
            if arr.has_object() {
                format!("{:>20}\n{}",
                        "[object]",
                        pretty_print_object(arr.object(), &format!("[{}]", prefix)))
            } else {
                format!("{}", arr)
            }
        }
        Object(ref obj) => pretty_print_object(obj, prefix),
        Null => "<null>".to_string(),
    }
}


fn pretty_print_object(object: &Object, prefix: &str) -> String {
    let mut output = String::new();

    let mut keys = object.keys().collect::<Vec<&String>>();
    keys.sort();
    for (i, k) in keys.iter().enumerate() {
        let next_prefix = if prefix.is_empty() {
            k.to_string()
        } else {
            format!("{}.{}", prefix, k)
        };

        output.push_str(&format!("{:<60}", next_prefix));

        output.push_str(&format!("{:>20}", pretty_print(object.get(k).unwrap(), &next_prefix)));

        if i < keys.len() - 1 {
            output.push('\n');
        }
    }
    output
}

extern crate argparse;
extern crate serde_json;

use argparse::{ArgumentParser, StoreOption, StoreTrue};
use serde_json::{Value, Number};
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{Read, stdin};
use std::iter::FromIterator;


#[derive(Debug)]
enum Case {
    Value(Type),
    Values(HashSet<Type>),
    Array(Vec<Case>),
    Object(HashMap<String, Case>),
    Null,
    Multi(Vec<Case>),
}

impl Case {
    fn from_number(number: Number) -> Case {
        if number.is_f64() {
            Case::Value(Type::Float)
        } else {
            Case::Value(Type::Int)
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Type {
    Float,
    Int,
    String,
    Boolean,
}

fn main() {
    let mut verbose = false;
    let mut input_path: Option<String> = None;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut input_path)
            .add_argument("file", StoreOption, "Name for the greeting");
        ap.parse_args_or_exit();
    }

    let result = if input_path.is_none() {
        process_input(stdin())
    } else {
        process_input(File::open(input_path.unwrap()).unwrap())
    };
    println!("{:#?}", result);
}



fn process_input<Source: Read + Sized>(input: Source)
    where Source: Read
{

    let input = BufReader::new(input);

    let mut case = Case::Null;

    for line in input.lines() {
        let line = line.unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        let new_case = process_element(v);
        case = merge_cases(case, new_case);

    }

    println!("{}", pretty_print(&case));
}



fn process_element(value: Value) -> Case {
    match value {
        Value::Object(map) => {
            let mut object_map: HashMap<String, Case> = Default::default();

            for (key, value) in map {
                let children = process_element(value);
                object_map.insert(key, children);
            }
            Case::Object(object_map)
        }
        Value::Null => Case::Null,
        Value::Bool(_) => Case::Value(Type::Boolean),
        Value::Number(number) => Case::from_number(number),
        Value::String(_) => Case::Value(Type::String),
        Value::Array(values) => {
            Case::Array(Vec::from_iter(values.into_iter().map(process_element)))
        }
    }
}


fn merge_cases(case_a: Case, case_b: Case) -> Case {
    use Case::*;

    match (case_a, case_b) {
        (Value(val_a), Value(val_b)) => {
            if val_a == val_b {
                Value(val_a)
            } else {
                let mut set: HashSet<Type> = Default::default();
                set.insert(val_a);
                set.insert(val_b);
                Values(set)
            }
        }
        (Values(mut vals), Value(val)) => {
            vals.insert(val);
            Values(vals)
        }
        (Value(val), Values(mut vals)) => {
            vals.insert(val);
            Values(vals)
        }
        (Values(vals_a), Values(vals_b)) => Values(vals_a.union(&vals_b).cloned().collect()),
        (Null, a) => a,
        (b, Null) => b,
        (Object(obj_a), Object(obj_b)) => merge_objects(obj_a, obj_b),
        (Array(arr), Value(val)) => Multi(vec![Array(arr), Value(val)]),
        (Array(arr), Values(vals)) => Multi(vec![Array(arr), Values(vals)]),
        (Value(val), Array(arr)) => Multi(vec![Array(arr), Value(val)]),
        (Value(val), Object(obj)) => Multi(vec![Object(obj), Value(val)]),
        (Values(vals), Array(arr)) => Multi(vec![Array(arr), Values(vals)]),
        (Values(vals), Object(obj)) => Multi(vec![Object(obj), Values(vals)]),
        (Array(arr_a), Array(arr_b)) => merge_arrays(arr_a, arr_b),
        (Array(arr), Object(obj)) => Multi(vec![Object(obj), Array(arr)]),
        (Object(obj), Value(val)) => Multi(vec![Object(obj), Value(val)]),
        (Object(obj), Values(vals)) => Multi(vec![Object(obj), Values(vals)]),
        (Object(obj), Array(arr)) => Multi(vec![Object(obj), Array(arr)]),
        (Multi(mut multi_a), Multi(multi_b)) => {
            multi_a.extend(multi_b);
            Multi(multi_a)
        }
        (Multi(mut multi_a), smt) => {
            multi_a.push(smt);
            Multi(multi_a)
        }
        (smt, Multi(mut multi_a)) => {
            multi_a.push(smt);
            Multi(multi_a)
        }
        // _ => Null,
    }
}


fn merge_objects(mut obj_a: HashMap<String, Case>, obj_b: HashMap<String, Case>) -> Case {
    for (k,v) in obj_b{
        obj_a.insert(k, v);
    }
    return Case::Object(obj_a)
}
fn merge_arrays(arr_a: Vec<Case>, arr_b: Vec<Case>) -> Case {
    unimplemented!();
}





fn pretty_print(case: &Case) -> String {
    use Case::*;

    let mut output = String::new();

    match *case {
        Value(ref val) => return format!("{:#?}", val).to_lowercase(),
        Values(ref vals) => return format!("{:#?}", vals).to_lowercase(),
        Array(ref arr) => return format!("{:#?}", arr),
        Object(ref obj) => {
            let mut keys = obj.keys().collect::<Vec<&String>>();
            keys.sort();

            for k in keys {
                output.push_str(&format!("{:<60}", k));
                output.push_str(&format!("{:>20}", pretty_print(obj.get(k).unwrap())));
                output.push('\n');
            }
            "".to_string()
        }
        Null => return "<null>".to_string(),
        Multi(ref cases) => String::new(),
    };

    return output;
}

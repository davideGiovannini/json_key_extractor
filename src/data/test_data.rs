use super::*;

use quickcheck::{empty_shrinker, quickcheck, Arbitrary, Gen};

impl Arbitrary for Type {
    fn arbitrary(g: &mut Gen) -> Type {
        let choices = [Type::Boolean, Type::String, Type::Int, Type::Float];
        g.choose(&choices).unwrap().clone()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Type>> {
        empty_shrinker()
    }
}

quickcheck! {
    fn add_value_to_value(type_a: Type, type_b: Type) -> bool {
        if type_a == type_b{
          Values::new(type_a) + Values::new(type_b) == Values::new(type_a)
        }else{
          Values::new(type_a) + Values::new(type_b) == Values::from_values(&[type_a, type_b])
        }
    }
}

fn date() -> Case {
    Case::Values(Values::new(Type::Date))
}

fn string() -> Case {
    Case::Values(Values::new(Type::String))
}

#[test]
fn simple_date() {
    assert_eq!(Case::from_string("2017-12-10"), date());
    assert_eq!(Case::from_string("2017/12/10"), date());
    assert_eq!(Case::from_string("2017_12_10"), date());
    assert_eq!(Case::from_string("2017\\12\\10"), date());
    assert_eq!(Case::from_string("2017.12.10"), date());
    assert_eq!(Case::from_string("2017+12+10"), date());
    assert_eq!(Case::from_string("2017:12:10"), date());
    assert_eq!(Case::from_string("2017 12 10"), date());
}

#[test]
fn noisy_date() {
    assert_eq!(Case::from_string("2017-12-10  "), date());
    assert_eq!(Case::from_string("   2017-12-10"), date());
    assert_eq!(Case::from_string("   2017-12-10   "), date());
}

#[test]
fn timestamps() {
    assert_eq!(Case::from_string("2017-05-21T07:36:12.161014"), date());
    assert_eq!(Case::from_string("2017-05-21T07:36:12"), date())
}

#[test]
fn noisy_timestamps() {
    assert_eq!(Case::from_string("   2017-05-21T07:36:12.161014"), date());
    assert_eq!(Case::from_string("2017-05-21T07:36:12.161014    "), date());
    assert_eq!(
        Case::from_string("     2017-05-21T07:36:12.161014   "),
        date()
    );

    assert_eq!(Case::from_string("   2017-05-21T07:36:12"), date());
    assert_eq!(Case::from_string("2017-05-21T07:36:12    "), date());
    assert_eq!(Case::from_string("   2017-05-21T07:36:12    "), date());
}

#[test]
fn string_containing_dates_are_still_strings() {
    assert_eq!(Case::from_string("2017-05-01 blabla"), string());
    assert_eq!(Case::from_string("Starting from 2017-05-01"), string());
    assert_eq!(
        Case::from_string("Starting from 2017-05-01 blabla"),
        string()
    );
    assert_eq!(Case::from_string("bb2017-05-01aaa"), string());
}

#[test]
fn wrong_dates_are_string() {
    assert_eq!(Case::from_string("42017-05-01"), string());
    assert_eq!(Case::from_string("2017-205-01"), string());
    assert_eq!(Case::from_string("2017-05-013"), string());
    assert_eq!(Case::from_string("42017-205-013"), string());

    assert_eq!(Case::from_string("2017305213"), string());
    assert_eq!(Case::from_string("2017f05b01"), string());
}

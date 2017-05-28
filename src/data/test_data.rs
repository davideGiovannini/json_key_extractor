

use super::Type;
use super::Values;

use quickcheck::{Arbitrary, Gen, empty_shrinker};

impl Arbitrary for Type {
    fn arbitrary<G: Gen>(g: &mut G) -> Type {
        let choices = [Type::Boolean, Type::String, Type::Int, Type::Float];
        *g.choose(&choices).unwrap()
    }

    fn shrink(&self) -> Box<Iterator<Item = Type>> {
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

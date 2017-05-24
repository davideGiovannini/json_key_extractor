

use super::Case;
use super::Case::*;
use super::Type;

use quickcheck::{Arbitrary, Gen, empty_shrinker};

impl Arbitrary for Type {
    fn arbitrary<G: Gen>(g: &mut G) -> Type {
      let choices = [Type::Boolean, Type::String, Type::Int, Type::Float];
        *g.choose(&choices ).unwrap()
    }

    fn shrink(&self)  -> Box<Iterator<Item=Type>> {
        empty_shrinker()
    }
}


quickcheck! {
      fn prop(type_a: Type, type_b: Type) -> bool {
        println!("{:?}", type_a);
          if type_a == type_b{
            Value(type_a) + Value(type_b) == Value(type_a)
          }else{
            Value(type_a) + Value(type_b) == Case::new_values(type_a, type_b)
          }
      }
  }

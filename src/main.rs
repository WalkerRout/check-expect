macro_rules! check_expect {
  ($test_name:ident -> $expr1:expr => $expr2:expr) => {
    #[test]
    fn $test_name() {
      assert_eq!($expr1, $expr2);
    }
  };
}

// for longer unit tests
check_expect!(add_0_1 -> {
  let a = 0;
  let b = 1;
  add(a, b)
} => 1);
check_expect!(add_2_3 -> add(2, 3) => 5);
check_expect!(add_4_5 -> add(4, 5) => 9);
check_expect!(add_0_0 -> add(0, 0) => 0);
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

pub fn main() {
  let x = add(1, 2);
  println!("{x}");
}

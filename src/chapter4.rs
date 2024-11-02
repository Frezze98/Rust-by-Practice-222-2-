/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test411() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32;

    let z = 10;

    println!("Success!");
}
#[test]
fn test412() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
#[test]
fn test413() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}}
#[test]
fn test414() {
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())

    }
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}
#[test]
fn test415() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i16::checked_add(251_i16, 8).unwrap();
    println!("{}, {}", v1, v2);
}
#[test]
fn test416() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("Success! v = {}", v);
}
#[test]
fn test417() {
    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test418() {
    assert!(0.1 as f32 + 0.2 as f32 == 0.3_f32);

    println!("Success!");
}
#[test]
fn test419() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -5);
    for c in 'a'..='z' {
        println!("{} - {}", c as u32, c as u32);
    }
}
#[test]
fn test4110() {
    use std::ops::{Range, RangeInclusive};


    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
#[test]
fn test4111() {
    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 1e-10);
    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

///https://practice.course.rs/basic-types/char-bool-unit.html
#[test]
fn test421() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}
#[test]
fn test422() {
    let c1 = '中';
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}
#[test]
fn test423() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}
#[test]
fn test424() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
#[test]
fn test425() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
#[test]
fn test426() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

///https://practice.course.rs/basic-types/statements-expressions.html
#[test]
fn test431() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}
#[test]
fn test432() {
    let v = {
        let x = 3;
        x
    };
    assert!(v == 3);

    println!("Success!");
}
#[test]
fn test433() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
    // i removed the semicolon to make it an expression, then it will return a value
}

///https://practice.course.rs/basic-types/functions.html
#[test]
fn test442() {
    print();
fn print() {
    println!("Success!");
   }
}
#[test]
fn test443() {
    fn main() {
        never_return();
        println!("Failed!");
    }

    fn never_return() -> ! {
        loop {
        }
    }

}
#[test]
fn test444() {
    println!("Success!");


fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}
fn never_return_fn() -> ! {

    unimplemented!()
   }
}
#[test]
fn test445() {
    // FILL in the blank
    let b = true;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}




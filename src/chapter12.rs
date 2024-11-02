///https://practice.course.rs/type-conversions/as.html
#[test]
fn test1211() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8; // 97

    let c1: char = decimal as u8 as char; // 97 -> a
    let c2 = integer as char; // a

    assert_eq!(integer + 1, 'b' as u8);

    println!("Success!");
}
#[test]
fn test1212() {
    assert_eq!(u8::MAX, 255);
    let v = 255;

    println!("Success!");
}
#[test]
fn test1214() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address = first_address + 4;
    let p2: *mut i32 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}
#[test]
fn test1215() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13)
    }

    println!("Success!");
}

///https://practice.course.rs/type-conversions/from-into.html
#[test]
fn test1221() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a'.into();

    let s: String = 'a'.into();

    println!("Success!");
}
#[test]
fn test1222() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(n: i32) -> Number {
            Number { value: n }
        }
    }
     let num = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30_i32.into();
        assert_eq!(num.value, 30);

        println!("Success!");
}
#[test]
fn test1223() {
    use core::num;
    use std::fs;
    use std::io;

    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(e: io::Error) -> CliError {
            CliError::IoError(e)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(e: num::ParseIntError) -> CliError {
            CliError::ParseError(e)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        let contents = fs::read_to_string(&file_name)?;
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }

        println!("Success!");
}
#[test]
fn test1224() {
    let n: i16 = 256;

    let n: u8 = match n.try_into() {
        // or: u8::try_from(n)
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}
#[test]
fn test1225() {
    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));

        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

        println!("Success!");
}

///https://practice.course.rs/type-conversions/others.html
#[test]
fn test1231() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");

        println!("Success!");
}
#[test]
fn test1232() {
    use std::str::FromStr;
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        let from_str = i32::from_str("20").unwrap();
        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);

        println!("Success!");
}
#[test]
fn test1233() {
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl FromStr for Point {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        }
    }
        let p = "(3,4)".parse::<Point>();
        assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

        println!("Success!");
}

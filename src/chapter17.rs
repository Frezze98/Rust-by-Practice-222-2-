///https://practice.course.rs/lifetime/basic.html
#[test]
fn test1711() {
    let i = 3;
    {
        let borrow1 = &i;
        //
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}
#[test]
fn test1712() {
    {
        let r;

        {
            let x = 5;
            r = &x;

            println!("r: {}", r);
        }
    }
}
#[test]
fn test1713() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
#[test]
fn test1714() {
    fn invalid_output() -> String {
        String::from("foo")
    }
}
#[test]
fn test1715() {
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 12;

        let y: &i32 = &_x;

    }
        let (four, nine) = (4, 9);

        print_refs(&four, &nine);

        failed_borrow();
}
#[test]
fn test1716() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
     let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
}
#[test]
fn test1717() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

        let var_a = 35;
        let example: Example;

        let var_b = NoCopyType {};

        /* fixme */
        example = Example {
            a: &var_a,
            b: &var_b,
        };

        println!("(Success!) {:?}", example);
}
#[test]
fn test1718() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
        foo.b
    }

        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!")
}
#[test]
fn test1719() {
    struct ImportantExcerpt {
        part: &'static str,
    }

    impl ImportantExcerpt {
        fn level(&self) -> i32 {
            3
        }
    }
}
#[test]
fn test17110() {
    fn input(x: &i32) {
        println!("`annotated_input`: {}", x);
    }

    fn pass(x: &i32) -> &i32 {
        x
    }

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        x
    }

    struct Owner(i32);

    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one(&mut self) {
            self.0 += 1;
        }
        fn print(&self) {
            println!("`print`: {}", self.0);
        }
    }

    struct Person {
        age: u8,
        name: &'static str,
    }

    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
}

///https://practice.course.rs/lifetime/static.html
#[test]
fn test1721() {
    static V: &str = "hello";
    need_static(V);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");

}
#[test]
fn test1722() {
    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }

    static mut CONFIG: Option<&'static mut Config> = None;

    fn init() -> Option<&'static mut Config> {
        Some(Box::leak(Box::new(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        })))
    }

        unsafe {
            CONFIG = init();

            println!("{:?}", CONFIG);
        }

}
#[test]
fn test1723() {
        let static_string = "I'm in read-only memory";
        {
            println!("static_string: {}", static_string);

        }

        println!("static_string reference remains alive: {}", static_string);

}
#[test]
fn test1724() {
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

        {
            let lifetime_num = 9;

            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);

}
#[test]
fn test1725() {
    use std::fmt::Debug;

    fn print_it<T: Debug + 'static>(input: T) {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it1(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it2<T: Debug>(input: &T) {
        println!("Value passed in is: {:?}", input);
    }

        let i = 5;
        print_it(i);

        print_it2(&i);

        print_it1(i);

        print_it2(&i);

}
#[test]
fn test1726() {
    use std::fmt::Display;

        let mut string = "First".to_owned();

        string.push_str(string.to_uppercase().as_str());
        print_a(&string);
        print_b(&string);
        // print_c(&string); // Compilation error
        // print_d(&string); // Compilation error
        print_e(&string);
        print_f(&string);
        // print_g(&string); // Compilation error

    fn print_a<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }

    fn print_b<T>(t: &T)
    where
        T: Display + 'static,
    {
        println!("{}", t);
    }

    fn print_c(t: &'static dyn Display) {
        println!("{}", t)
    }

    fn print_d(t: &'static impl Display) {
        println!("{}", t)
    }

    fn print_e(t: &(dyn Display + 'static)) {
        println!("{}", t)
    }

    fn print_f(t: &(impl Display + 'static)) {
        println!("{}", t)
    }

    fn print_g(t: &'static String) {
        println!("{}", t);
    }

}

///https://practice.course.rs/lifetime/advance.html
#[test]
fn test1731() {
    struct DoubleRef<'a, 'b: 'a, T> {
        r: &'a T,
        s: &'b T,
    }
     println!("Success!")
}
#[test]
fn test1732() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
     let excerpt = ImportantExcerpt {
            part: "This is an important excerpt.",
        };

        let announcement = "Here comes the announcement!";
        let part = excerpt.announce_and_return_part(announcement);

        println!("Part: {}", part);
        println!("Success!")

}
#[test]
fn test1733() {
    fn f<'a>(x: &'a i32, y: &mut &'a i32) {
        *y = x;  // Присвоюємо `y` значення `x`
    }

        let x = 42;
        let mut y = &0;
        f(&x, &mut y);

        println!("y: {}", y);
}
#[test]
fn test1734() {
    /* Adding HRTB to make it work! */
    fn call_on_ref_zero<F>(f: F)
    where
            for<'a> F: Fn(&'a i32) {
        let zero = 0;
        f(&zero);
    }
     call_on_ref_zero(|x| println!("'static value passed in is: {}", x));

        println!("Success!");
}
#[test]
fn test1735() {
        let mut data = 10;
        let ref1 = &mut data;

        {
            // Перезапозичення ref1
            let ref2: &mut i32 = &mut *ref1;
            *ref2 += 2;
        }

        // Використання ref1 після перезапозичення
        *ref1 += 1;

        println!("{}", data); // Очікуваний результат: 13

}
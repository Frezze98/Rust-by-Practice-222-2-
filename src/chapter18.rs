///https://practice.course.rs/functional-programing/closure.html
#[test]
fn test1811() {
        let color = String::from("green");

        let print = || println!("`color`: {}", color);

        print();
        print();

        let _reborrow = &color;

        println!("{}",color);
}
#[test]
fn test1812() {
        let mut count = 0;

        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        inc();

        {
            let inc_closure = &mut inc;
            inc_closure();
        }

        assert_eq!(count, 2);

        println!("Success!");
}
#[test]
fn test1813() {
        let movable = Box::new(3);

        let consume = || {
            println!("`movable`: {:?}", movable);
            take(movable.clone());
        };

        consume();
        consume();
    fn take<T>(_v: T) {}
}
#[test]
fn test1814() {
        let example_closure = |x: String| x;

        let s = example_closure(String::from("hello"));

        let example_closure = |x: i32| x;
        let n = example_closure(5);
}
#[test]
fn test1815() {
    fn fn_once<F>(mut func: F)
    where
        F: FnMut(usize) -> bool,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    fn main() {
        let x = vec![1, 2, 3];
        fn_once(|z| { z == x.len() })
    }
}
#[test]
fn test1816() {
        let mut s = String::new();

        let update_string = |str: &str| s.push_str(str);

        exec(update_string);

        println!("{:?}", s);

    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }
}
#[test]
fn test1817() {
    fn apply<F>(f: F) where
        F: FnOnce() {

        f();
    }

    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {

        f(3)
    }
     use std::mem;

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        let diary = || {
            println!("I said {}.", greeting);

            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            mem::drop(farewell);
        };

        apply(diary);

        let double = |x| 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
}
#[test]
fn test1818() {
        let mut s = String::new();

        let update_string = |str: &str| -> String { s.push_str(str); s.clone() };

        exec(update_string);

        println!("{}", s);

    fn exec<F: FnMut(&str) -> String>(mut f: F) {
        f("hello");
    }
}
#[test]
fn test1819() {
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
}
#[test]
fn test18110() {
    fn create_fn() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }
    let fn_plain = create_fn();
    println!("{}", fn_plain(1));
}
#[test]
fn test18111() {
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {

        let num = 5;

        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x + num)
        }
    }

        let closure = factory(2);
        println!("{}", closure(3));

}

///https://practice.course.rs/functional-programing/iterator.html
#[test]
fn test1821() {
        let arr = [0; 10];
        arr.iter().for_each(|&x| println!("{}", x));
}
#[test]
fn test1822() {
        let mut v = Vec::new();
        for n in 0..100 {
            v.push(n);
        }

        assert_eq!(v.len(), 100);
}
#[test]
fn test1823() {
        let v1 = vec![1, 2];
        let mut iter = v1.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
}
#[test]
fn test1824() {
        let arr = vec![0; 10];
        for i in &arr {
            println!("{}", i);
        }

        println!("{:?}", arr);
}
#[test]
fn test1825() {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
}
#[test]
fn test1826() {
        let mut values = vec![1, 2, 3];
        let mut values_iter = values.iter_mut();

        if let Some(v) = values_iter.next() {
            *v = 0;
        }

        assert_eq!(values, vec![0, 2, 3]);
}
#[test]
fn test1827() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

        let mut fib = fibonacci();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
}
#[test]
fn test1828() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);

        println!("{:?}", v1);
}
#[test]
fn test1829() {
    use std::collections::HashMap;

        let names = [("sunface", 18), ("sunfei", 18)];
        let folks: HashMap<_, _> = names.into_iter().collect();
        println!("{:?}", folks);

        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<i32> = v1.iter().cloned().collect();

        assert_eq!(v2, vec![1, 2, 3]);
}
#[test]
fn test18210() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|&x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
}
#[test]
fn test18211() {
    use std::collections::HashMap;

        let names = ["sunface", "sunfei"];
        let ages = [18, 18];
        let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

        println!("{:?}", folks);
}
#[test]
fn test18212() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
    }
     let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
}
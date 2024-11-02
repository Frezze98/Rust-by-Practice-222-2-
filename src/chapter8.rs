///https://practice.course.rs/pattern-match/match-iflet.html
#[test]
fn test811() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North| Direction::South  => {
                println!("South or North");
            },
            _ => println!("Direction is West"),
        };
}
#[test]
fn test812() {
    let boolean = true;

    let binary = match boolean{
        true => 1,
        _ => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}
#[test]
fn test813() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(r, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants")
        }
    }
}
#[test]
fn test814() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z'| 'A'..='Z'| '0'..='9'))
    }

    println!("Success!");
}
#[test]
fn test815() {
    enum MyEnum {
        Foo,
        Bar
    }

        let mut count = 0;

        let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
        for e in v {
            if matches!(e,MyEnum::Foo){
                count += 1;
            }
        }

        assert_eq!(count, 2);

        println!("Success!");
}
#[test]
fn test816() {
    let o = Some(7);

    if let Some(i) = o {

        println!("This is a really long string and `{:?}`", i);

        println!("Success!");
    }
}
#[test]
fn test817() {
    enum Foo {
        Bar(u8)
    }

        let a = Foo::Bar(1);

        if let Foo::Bar(i) = a {
            println!("foobar holds the value: {}", i);

            println!("Success!");
        }
}
#[test]
fn test818() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }


        let a = Foo::Qux(10);

        match a {
            Foo::Bar => println!("match foo::bar"),
            Foo::Baz => println!("match foo::baz"),
            _ => println!("match others")
        }
}
#[test]
fn test819() {
    let age = Some(30);
    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}

///https://practice.course.rs/pattern-match/patterns.html
#[test]
fn test821() {
    fn match_number(n: i32) {
        match n {
            1 => println!("One!"),
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            6..=10 => {
                println!("match 6 -> 10")
            },
            _ => {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }
}
#[test]
fn test822() {
    struct Point {
        x: i32,
        y: i32,
    }

        let p = Point { x: 2, y: 30 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
}
#[test]
fn test823() {
    enum Message {
        Hello { id: i32 },
    }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id @ 3..=7,
            } => println!("Found an id in range [3, 7]: {}", id),
            Message::Hello { id: newid @ (10 | 11 | 12)} => {
                println!("Found an id in another range [10, 12]: {}", newid)
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
}
#[test]
fn test824() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}
#[test]
fn test825() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}
#[test]
fn test826() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!")
    }
}

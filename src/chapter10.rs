///https://practice.course.rs/generics-traits/generics.html
#[test]
fn test1011() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(7));

        generic::<char>(SGen('A'));

        generic(SGen('Z'));

        println!("Success!");
}
#[test]
fn test1012() {
    fn sum <T: std::ops::Add<Output = T>>(a:T, b:T) -> T{
        return a + b
    }

        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
}
#[test]
fn test1013() {
    struct Point<T>{
        x: T,
        y: T,
    }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        println!("Success!");
}
#[test]
fn test1014() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

        let p = Point{x: 5, y : "hello".to_string()};

        println!("Success!");
}
#[test]
fn test1015() {
    struct Val<T>{
        val: T,
    }

    impl<T> Val<T>{
        fn value(&self) -> &T {
            &self.val
        }
    }

        let x = Val{ val: 3.0 };
        let y = Val{ val: "hello".to_string()};
        println!("{}, {}", x.value(), y.value());
}
#[test]
fn test1016() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<P,Q>(self, other:Point<P,Q>) -> Point<T,Q>{
            Point { x: self.x, y: other.y }
        }
    }

        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中'};

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
}
#[test]
fn test1017() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

        let p = Point{x: 5.0, y: 10.0};
        println!("{}",p.distance_from_origin());
}

///https://practice.course.rs/generics-traits/const-generics.html
#[test]
fn test1021() {
    struct Array<T, const N: usize> {
        data: [T; N],
    }

        let arrays: [Array<_, 3>; 3] = [
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 0],
            },
        ];

        println!("Success!");
}
#[test]
fn test1022() {
    fn print_array<T: std::fmt::Debug>(arr: &[T]){
        println!("{:?}", arr);
    }

        let arr = [1, 2, 3];
        print_array(&arr);

        let arr = ["hello", "world"];
        print_array(&arr);
}
#[test]
fn test1023() {
    fn check_size<T>(val: T)
    where
        T: Sized,
    {
        assert!(core::mem::size_of::<T>() < 768, "Size constraint violated");
        // ...
    }

        check_size([0u8; 767]);
        check_size([0i32; 191]);
        check_size("hello你好");
        check_size([(); 20].iter().map(|_| "hello你好".to_string()).collect::<Vec<_>>());
        check_size(['中'; 5]);

        println!("Success!");
}

///https://practice.course.rs/generics-traits/traits.html
#[test]
fn test1031() {
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }

        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }

        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!");
}
#[test]
fn test1032() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = (_one_second == _one_second);
        let _this_is_true = (_one_second > _one_second);

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
}
#[test]
fn test1033() {
    use std::ops;

    fn multiply<T>(a: T, b: T) -> T
    where
        T: std::ops::Mul<Output = T>,
    {
        a * b
    }

        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));

        println!("Success!");
}
#[test]
fn test1034() {
    use std::ops;

    struct Foo;
    struct Bar;
    #[derive(Debug, PartialEq)]
    struct FooBar;
    #[derive(Debug, PartialEq)]
    struct BarFoo;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }

        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!");
}
#[test]
fn test1035() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);

    fn summary(t: &impl Summary) {
        println!("{}", t.summarize());
    }
}
#[test]
fn test1036() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    fn random_animal<'a>(random_number: f64) -> &'a dyn Animal {
        if random_number < 0.5 {
            &Sheep {}
        } else {
            &Cow {}
        }
    }

        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!(
            "You've randomly chosen an animal, and it says {}",
            animal.noise()
        );
}
#[test]
fn test1037() {
    assert_eq!(sum(1, 2), 3);
}
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
#[test]
fn test1038() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Unit(i32);

        let pair = Pair {
            x: Unit(1),
            y: Unit(3),
        };

        pair.cmp_display();
}
#[test]
fn test1039() {
    fn example1() {
        // `T: Trait` is the commonly used way.
        // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }

        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.value(10), 11);
        assert_eq!(cacher.value(15), 11);
    }

    fn example2() {
        // We can also use `where` to construct `T`
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 21);
    }

        example1();
        example2();

        println!("Success!");
}

///https://practice.course.rs/generics-traits/trait-object.html
#[test]
fn test1041() {
    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String {
            "duck duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String {
            "swan swan".to_string()
        }
    }

    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");

    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        match species {
            1 => Box::new(Swan),
            2 => Box::new(Duck),
            _ => panic!(),
        }
    }
}
#[test]
fn test1042() {
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

        let birds: [Box<dyn Bird>; 2] = [Box::new(Duck), Box::new(Swan)];

        for bird in birds {
            bird.quack();
        }
}
#[test]
fn test1043() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");

    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }

    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
}
#[test]
fn test1044() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }

    fn static_dispatch<T: Foo>(a: T) {
        a.method();
    }

    fn dynamic_dispatch(a: &dyn Foo) {
        a.method();
    }

        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!");
}
#[test]
fn test1045() {
    trait MyTrait {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32 {
        fn f(&self) -> u32 {
            42
        }
    }

    impl MyTrait for String {
        fn f(&self) -> String {
            self.clone()
        }
    }

    fn my_function(x: impl MyTrait) -> impl MyTrait {
        x.f()
    }

        my_function(13_u32);
        my_function(String::from("abc"));

        println!("Success!");
}

///https://practice.course.rs/generics-traits/advanced-traits.html
#[test]
fn test1051() {
    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!(
            "Does container contain {} and {}: {}",
            &number_1,
            &number_2,
            container.contains(&number_1, &number_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
}
#[test]
fn test1052() {
    use std::ops::Sub;

    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: Sub<Output = T>> Sub for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

        assert_eq!(
            Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
            Point { x: 1, y: 3 }
        );

        println!("Success!");
}
#[test]
fn test1053() {
    trait Pilot {
        fn fly(&self) -> String;
    }

    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }

    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }

        let person = Human;

        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");

        assert_eq!(person.fly(), "*waving arms furiously*");

        println!("Success!");
}
#[test]
fn test1054() {
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }

    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }

    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

        let student = CSStudent {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };

        println!("{}", comp_sci_student_greeting(&student));
}
#[test]
fn test1055() {
    use std::fmt;

    struct Pretty(String);

    impl fmt::Display for Pretty {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }

        let w = Pretty("hello".to_string());
        println!("w = {}", w);
}
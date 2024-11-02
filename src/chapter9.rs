///https://practice.course.rs/method.html
#[test]
fn test91() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Complete the area method which return the area of a Rectangle.
        fn area(self) -> u32{
            self.width * self.height
        }
    }

        let rect1 = Rectangle { width: 30, height: 50 };

        assert_eq!(rect1.area(), 1500);

        println!("Success!");
}
#[test]
fn test92() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self)  {
            println!("the current state is {}", self.color);
        }
    }
        let light = TrafficLight{
            color: "red".to_owned(),
        };
        light.show_state();
        println!("{:?}", light);
}
#[test]
fn test93() {
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self)  {
            println!("the current state is {}", self.color);
        }

        pub fn change_state(&mut self) {
            self.color = "green".to_string()
        }
    }
        println!("Success!");
}
#[test]
fn test94() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {

        pub fn new() -> TrafficLight{
            TrafficLight { color: String::from("red") }
        }


        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");

        println!("Success!");
}
#[test]
fn test95() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }


        println!("Success!");
}
#[test]
fn test96() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLightColor {
        fn color(&self) -> &str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

        let c = TrafficLightColor::Yellow;

        assert_eq!(c.color(), "yellow");

        println!("{:?}", c);

}

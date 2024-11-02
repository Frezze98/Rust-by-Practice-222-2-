///https://practice.course.rs/collections/string.html
#[test]
fn test1111() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
#[test]
fn test1112() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; //  or s.as_str()
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let mut slice3: String = s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}
#[test]
fn test1113() {
    let s: String = String::from("hello, world!");

    let slice: &str = &s;

    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
#[test]
fn test1114() {
    let s = String::from("hello, 世界");
    let slice1 = &s[..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate(){
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!");
}
#[test]
fn test1115() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    let s1 = String::from_utf8(v).unwrap();


    assert_eq!(s, s1);

    println!("Success!");
}
#[test]
fn test1116() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}
#[test]
fn test1117() {
    use std::mem;

        let story = String::from("Rust By Practice");

        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_ptr();
        let len = story.len();
        let capacity = story.capacity();

        assert_eq!(16, len);

        let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) };

        assert_eq!(*story, s);

        println!("Success!");
}

///https://practice.course.rs/collections/vector.html
#[test]
fn test1121() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    let mut v1 = vec![];

    for i in &v {
        v1.push(*i);
    }
    is_vec(&v1);

    assert_eq!(v, v1);

    println!("Success!");

    fn is_vec(v: &Vec<u8>) {}
}
#[test]
fn test1122() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!");
}
#[test]
fn test1123() {
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);

    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}
#[test]
fn test1124() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("Iteration #{i}: {:?}", v.get(i))
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2),
        }
    }

    assert_eq!(format!("{:?}", v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!")
}
#[test]
fn test1125() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}
#[test]
fn test1126() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    println!("{:?}", vec);

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    println!("{:?}", vec);

    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("{:?}", vec);

    println!("Success!");
}
#[test]
fn test1127() {
    #[derive(Debug, PartialEq)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
        let ip_v4: String = String::from("127.0.0.1");
        let ip_v6: String = String::from("::1");
        let v: Vec<IpAddr> = vec![IpAddr::V4(ip_v4), IpAddr::V6(ip_v6)];

        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));

        println!("Success!");
}
#[test]
fn test1128() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v {
            ip.display();
        }
}

///https://practice.course.rs/collections/hashmap.html
#[test]
fn test1131() {
    use std::collections::HashMap;

        let mut scores: HashMap<&str, i32> = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
}
#[test]
fn test1132() {
    use std::collections::HashMap;

        let teams: [(&str, i32); 3] = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        let teams_map2 = teams.into_iter().collect();
        assert_eq!(teams_map1, teams_map2);

        println!("Success!");
}
#[test]
fn test1133() {
    use std::collections::HashMap;
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");

    fn random_stat_buff() -> u8 {
        42
    }
}
#[test]
fn test1134() {
    use std::collections::HashMap;

    #[derive(Debug, Eq, Hash, PartialEq)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }
     let vikings: HashMap<Viking, i32> = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
}
#[test]
fn test1135() {
    use std::collections::HashMap;
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap : {}", v1);

        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();
        m2.insert(v2.as_str(), v1);

        assert_eq!(v2, "hello");

        println!("Success!");
}
///https://practice.course.rs/flow-control.html
#[test]
fn test71() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}
#[test]
fn test72() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };

    println!("{} -> {}", n, big_n);
}
#[test]
fn test73() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}
#[test]
fn test74() {
    let names: [String; 2]= [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        println!("{:?}", name);
    }

    println!("{:?}", names);

    let numbers :[i32; 3]= [1, 2, 3];
    for n in numbers {
        println!("{:?}", n);
    }

    println!("{:?}", numbers);
}
#[test]
fn test75() {
    let a = [4, 3, 2, 1];

    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}
#[test]
fn test76() {
    let mut n = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n += 1;
    }

    println!("n reached {}, so loop is over",n);
}
#[test]
fn test77() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
#[test]
fn test78() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
#[test]
fn test79() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}
#[test]
fn test710() {

        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);

        println!("Success!");
}
#[test]
fn test711() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}

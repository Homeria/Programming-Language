use rust_hw1::problems::*;

fn main() {
    // sum 함수 테스트 케이스
    assert_eq!(sum(-10), 0);
    assert_eq!(sum(100), 5050);

    // factorial 함수 테스트 케이스
    assert_eq!(factorial(-10), 0);
    assert_eq!(factorial(5), 120);

    // circle 함수 테스트 케이스
    assert_eq!(circle(-10.1), 0.0);
    assert_eq!(circle(4.2), 55.389595);

    // concat 함수 테스트 케이스
    assert_eq!(concat("Bob!"), "Hello Bob!");
    assert_eq!(concat("Alice!"), "Hello Alice!");

    // xor 함수 테스트 케이스
    assert_eq!(xor(true, true), false);
    assert_eq!(xor(true, false), true);
    assert_eq!(xor(false, true), true);
    assert_eq!(xor(false, false), false);

    // max 함수 테스트 케이스
    assert_eq!(max([1, 2, 3, 4, 5]), 5);
    assert_eq!(max([5, 4, 3, 2, 1]), 5);

    // triangle 함수 테스트 케이스
    assert_eq!(triangle(-3, 3, 1), false);
    assert_eq!(triangle(3, 4, 5), true);
    assert_eq!(triangle(100, 1, 2), false);

    // if_else 함수 테스트 케이스
    assert_eq!(if_else(true, 2, 100), 102);
    assert_eq!(if_else(100 < 2, 2, -2), 4);

    // calculator 함수 테스트 케이스
    assert_eq!(calculator(1.0, "+", 3.0), 4.0);
    assert_eq!(calculator(3.0, "-", 1.0), 2.0);
    assert_eq!(calculator(2.0, "*", 4.0), 8.0);
    assert_eq!(calculator(3.0, "/", 1.5), 2.0);

    // distance 함수 테스트 케이스
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 4.0, y: 6.0 };
    assert_eq!(distance(a, b), 5.0);

    println!("Success!");
}

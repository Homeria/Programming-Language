fn main() {

    // 변수와 가변성
        // Rust에서 변수는 let 문을 사용하여 정의
        // 기본 선언은 불변성을 가짐 -> 의도치 않게 값을 변경하는 것을 방지
        let x1 = 5;
        println!("변수와 가변성 - The value of x is: {x1}");
        
        // [x1 = 6] => [cannot mutate immutable variable 'x1'] Error 발생
        // mut 키워드를 통해 가변성을 가질 수 있음
        let mut x2 = 5;
        println!("변수와 가변성 - The value of x2 is: {x2}");
        x2 = 6;
        println!("변수와 가변성 - The value of x2 is: {x2}");

        // 상수
            // 상수는 번경이 허용되지 않는 것이며, 코드의 모든 범위에서 선언 가능
            // 상수는 값의 유형을 선언해야 하며, mut 키워드 사용이 불가함.
            // 상수는 런타임에서 결정되는 값을 사용 불가함
            const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        
        // shadowing
            // 같은 이름의 새 변수를 선언하는 것
            // 선언된 변수의 Scope가 끝날 때까지 유지 가능
            // mut과 달리 변수 타입 변경 가능
            let x3 = 5;
            let x3 = x3 + 1;
            {
                let x3 = x3 * 2;
                println!("shadowing - The value of x3 in the inner scope is: {x3}");
                // 이 괄호 안 x3는 괄호 안의 고유 범위에서만 적용되는 지역 변수의 개념
                // 따라서, 괄호 안 x3는 변하되 괄호 밖 x3는 변하지 않음
            }
            println!("shadowing - The value of x3 is: {x3}");


        // Rust는 타입이 고정된 언어

            // 컴파일 시 변수의 타입을 반드시 알 수 있어야 함
            // 다양한 타입이 선택될 수 있는 경우 타입을 명시해야 함
            let guess1: u32 = "42".parse().expect("Not a number!");

            // "42"는 i32, u32, 문자열 등 다양한 타입이 될 수 있음
            // let guess2 = "42".parse().expect("Not a number!"); => Type 기입해야 함


    // 데이터 타입

        // 정수형 타입
            // Length   Signed  Unsigned
            // 8-bit    i8      u8
            // 16-bit   i16     u16
            // 32-bit   i32     u32
            // 64-bit   i64     u64
            // 128-bit  i128    u128
            // arch     isize   usize

            // Number literals  Example
            // Decimal          98_222
            // Hex              0xff
            // Octal            0o77
            // Binary           0b1111_0000
            // Byte (u8 only)   b'A'

        // 부동 소수점 타입
            // f32, f64
            let x4 = 2.0; // f64
            let x5: f32 = 3.0; // f32

        // 연산
            // 변수 선언 시 계산된 값을 기준으로 타입이 정해짐

            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;

            //remainder
            let remainder = 43 % 5;

        // Boolean 타입
            let t = true;
            let f: bool = false; // with explicit type annotation

        
        // 문자열 타입
            let c = 'z';
            let z = 'ℤ';
            let heart_eyed_cat = '😻';
            let abc = "abc";

        // Tuple 타입
            // 다양한 타입을 하나의 복합 타입으로 만드는 방법
            let tup: (i32, f64, u8) = (500, 6.4, 1);
            let (t1, t2, t3) = tup;
            println!("데이터 타입 - The value of t2 i is: {t2}");

            // Tuple 내부 값은 .index로 접근 가능
            let five_hundred = tup.0;
            let six_point_four = tup.1;
            let one = tup.2;

        // 배열
            // 단일 타입만 입력 가능
            // Rust의 배열은 선언한 뒤 크기 변경 부락
            // 배열 내부 값은 [index]로 접근 가능
            let arr = [1, 2, 3, 4, 5];
            println!("{}", arr[0]);


    // 실습
        //assert_eq! 는 값이 일치하는지 확인하는 코드드
        let p_x: i32 = 5;
        {
            let p_x = p_x + 7;
            // 또는 let p_x = 12;
            assert_eq!(p_x, 12);
        }
        assert_eq!(p_x, 5);
        println!("변수와 데이터 타입 실습 1 - Success!");

        let p_tup = ('a', 2, 'c');
        let p_arr = ['a', 'b', 'c'];
        let p_tup_ele = p_tup.1;
        let p_arr_ele = p_arr[1];
        assert!(p_tup_ele == 2);
        assert!(p_arr_ele == 'b');
        println!("변수와 데이터 타입 실습 2 - Success!");

    // 함수 동작 원리

        // 선언
            // fn 문을 사용하여 함수 정의
            another_function_1();

            // 모든 매게변수는 타입 정의가 필요
            // 여러 개의 매개변수는 쉼표로 구분분
            another_function_2(5, 6);
        
        // 반환값

            // 화살표를 사용하여 반환할 값의 타입 추가
            // 세미콜론이 없는 마지막 표현식 반환
            another_function_3(5, 6);

            // 값을 미리 반환하려면 return 문 사용
            println!("함수 동작 원리 - another_function_4: input:{}, output:{}", 0, another_function_4(0));
            println!("함수 동작 원리 - another_function_4: input:{}, output:{}", 1, another_function_4(1));
            
        // 구문과 표현식

            // 함수 본문문
                // 구문(Statement) - 명령어
                    // 선언문 (Declaration Statement)
                        // let x = 1;
                        // fn main() {}
                    // 값을 무시하는 표현식 (Express Statement)
                        // 함수에서 반환된 값을 사용하지 않는 경우 등

                // 표현식(Expression) - 값을 반환하는 코드
                    // a * b
                    // "string"
                    // 13
                    // add(1, 2)
            
            // 블록 표현식
            
            // 주석
                // - 한 줄
                // /* 여러 줄 */
    
    // 제어문
        // if 표현식
            // 조건에 따라 분기하도록 하는 표현식
                let number = 6;

                if number % 4 == 0 {
                    println!("제어문:if - The number is divisible by 4");
                } else if number % 3 == 0 {
                    println!("제어문:if - The number is divisible by 3");
                } else if number % 2 == 0 {
                    println!("제어문:if - The number is divisible by 2");
                } else {
                    println!("제어문:if - The number is not divisible by 4, 3, or 2");
                }
            // if 또한 표현식으로 아래와 같이 사용 가능
                let condition = true;
                let number = if condition {
                    5 
                } else {
                    6 
                };
                println!("제어문:if - The value of number is: {number}");
            // 여러 타입의 데이터가 주어지는 경우 타입을 결정할 수 없어 오류를 출력
            // 예를 들어, 위 if문에서 6 대신 "six"를 반환하는 것처럼 케이스마다 타입이 다르면 안됨

        // 반복문
            // loop문 : 조건없이 반복
                // loop {
                //     println!("제어문:loop - again!");
                // }

            // while문 : 조건부 반복
                let mut number = 3;
                while number != 0 {
                    println!("제어문:while - {}!", number);
                    number = number - 1;
                }
                println!("제어문:while - LIFTOFF!!!");
        
            // 리스트의 각 요소 순회하기
                let a = [10, 20, 30, 40, 50];
                
                // for .. in .. : 튜플은 요소의 타입이 다를 수 있어 순회 불가
                for element in a {
                    println!("제어문:for - the value is: {element}");
                }
                
                // for .. in .. : Range를 사용한 순회회
                for number in (1..4).rev() {
                    println!("제어문:for - {}!", number);
                }
                println!("제어문:for - LIFTOFF!!!");
                
            // 실습
                // sum_all 함수 작성하기
                let list = [1, 2, 3, 4, 5];
                let result = sum_all_1(list);
                assert_eq!(result, 15);
                println!("제어문 실습 - Success!");
                let result = sum_all_2(list);
                assert_eq!(result, 15);
                println!("제어문 실습 - Success!");
            

    // 소유권(Ownership)

        // 소유권이란?

            // Rust 프로그램이 메모리를 관리하는 규칙
                // 모든 Rust의 값에는 소유자가 있다
                // 단 하나의 소유자만이 존재할 수 있다
                // 소유자가 scope를 벗어나면 값은 삭제된다

            // 다른 언어와 달리 Garvage Collector가 없음
                // 소유권을 통해 더 이상 사용되지 않는 경우 메모리 삭제
                // 컴파일 단계에서 소유권에 대한 확인이 이루어짐

            // 변수 Scope
                // 변수가 선언된 블록 내에서만 유효
                // 블록을 벗어나면 변수는 삭제됨

        // String 타입
            // heap 메모리에 저장
                // 일반 문자열과 달리 수정이 가능
                // push_str()을 통해 문자열을 붙임
                    let mut s = String::from("Hello");
                    s.push_str((", World!"));
                    println!("소유권 - String 타입 : {s}");

            // Scope에서 벗어날 시 자동으로 메모리 할당 하제
                // Garbage 누수 문제 방지
                // Garbage Collector로 인한 성능 저하 X
                // 내부적으로 drop 함수 호출
                    let s = String::from("Hello");
                // scope(괄호)에서 벗어나면 drop 함수 자동 호출

        // 변수와 데이터 간 상호작용 : 이동
            // 정수형 등 단순한 데이터 타입은 단순 복사
                let x = 5;
                let y = x;
            // String 타입은 Heap에 저장된 데이터는 복사되지 않음
            // 문자열을 가리키는 포인터, 길이, 용량만 복사됨
                let s1 = String::from("hello");
                let s2 = s1;
                // s1의 ptr = s2의 ptr
                // s1의 len = s2의 len
                // s1의 capacity = s2의 capacity
                // s1의 소유권을 박탈하고 s2의 소유권을 주게 됨 == s1은 더 이상 유효하지 않음
            // Rust에서는 자동으로 deep copy(Heap에 저장된 데이터 복사)를 실시하지 않음

        // 변수와 데이터 간 상호작용 방식 : 클론
            // Heap에 저장된 데이터를 복사하려면 clone 사용용

        // 스택에만 저장되는 데이터 : 복사
            // 아래 데이터 타입은 copy trait을 구현하며, 항상 값을 복사함
                // 숫자 (정수형, 부동 소수점 타입)
                // Boolean
                // char (character)
                // Tuple (Copy Trait을 구현한 데이터만 포함된 경우)
        
        // 소유권과 함수
            // 함수에 변수를 전달하는 경우 변수 생성과 마찬가지로 이동이나 복사 과정이 진행됨됨
                let s = String::from("Hello");
                takes_ownership(s);
                //println!("{s}")

        // 반환 값과 Scope
            // 튜플 형식으로 소유권을 다시 돌려줄 수 있음
                let s1 = String::from("Hello");
                let (s2, len) = calculate_length(s1);
                println!("소유권 - 반환 값과 Scope : The length of '{s2}' is {len}");

        // 실습
            let s = String::from("Hello World");
            // 소유권 반환받기
            let s = print_str_1(s);
            // Parameter를 s의 clone으로 전달
            print_str_2(s.clone());
            println!("소유권 반환 - {}", s);

        // 참조와 대여
            // 소유권을 넘기는 대신 개체의 참조자 넘기기 - 대여(borrow)
            // 참조된 값은 수정 불가가
            // 가변 참조자를 사용하면 참조된 값 수정 가능
                let s1 = String::from("Hello");
                let len = calculate_length_2(&s1);
                println!("소유권 - 참조와 대여 : The length of '{s1}' is {len}");
            // 가변 참조자가 제한된 이유
                // 데이터 경합(Data Race) 방지
                    // 둘 이상의 포인터가 동시에 같은 데이터에 접근
                    // 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행
                    // 데이터 접근 동기화 메커니즘이 없음
                    // 이러한 경우 실행 순서에 따라 다른 결과가 발생할 수 있음
                // A와 B가 같은 데이터에 동시에 접근하는 경우
                // 다음과 같은 방법으로 메모리 안전성 보장
                    // 단 하나의 가변 참조자 혹은 여러 개의 불변 참조자
                    // 참조자는 항상 유효해야 함
            // 참조자 + 가변 참조자
                let mut s = String::from("hello");

                let r1 = &s; // no problem
                let r2 = &s; // no problem
                //let r3 = &mut s; // BIG PROBLEM
            // 가변 참조자 생성이 허용되는 경우
                // 이미 생성된 가변 참조자가 사라진 경우
                    let mut s = String::from("hello");
                    {
                        let r1 = &mut s;
                    }
                    // scope 이탈로 r1은 사라짐 -> 다시 가변 참조자 생성 가능능
                    let r2 = &mut s;
                // 가변 참조자 생성 이후 참조자가 사용되지 않는 경우
                    let mut s = String::from("hello");
                    let r1 = &s;
                    let r2 = &s;
                    println!("가변 참조자 생성 - {}, {}", r1, r2);
                    // variables r1 and r2 will not be used after this point
                    let r3 = &mut s;
                    println!("가변 참조자 - {}", r3);

            // Dangling Reference
                // 포인터가 남아있는 상태에서 메모리 해제
                // 참조자를 생성하는 대신 String을 직접 반환

            // 실습
                let mut s = String::from("hello, ");
                // 참조값만 줌
                let p = &mut s;
                p.push_str("world");
                println!("가변 참조자 실습 - Sucess!");
                
                let mut s = String::from("hello, ");
                let r1 = &mut s;
                r1.push_str("world");
                
                // 2번 이상 가변 참조할 수 없으므로 print를 지워야 함
                let r2 = &mut s;
                r2.push_str("!");
                //println!("{}", r1);
        // 슬라이스 타입
            // 컬렉션의 일부 요소만 참조하는 것
                // start .. end => [start <= x < end]
                // start .. => [start <= x]
                // .. end => [x <= end]
                // .. => [x]
                // start ..= end => [start <= x <= end]
                // ..= end => [x <= end]
                let s = String::from("hello world");
                let hello = &s[0..5]; // 0부터 5까지, 'h', 'e', 'l', 'l', 'o', ' '
                let world = &s[6..11]; // 6부터 11까지, 'w', 'o', 'r', 'l', 'd'
                println!("슬라이스 타입 - {}, {}", hello, world);

            // ex) 공백으로 구분되는 첫 단어 찾는 코드 - first_word 함수
            // 슬라이스가 있는 상태에서 내용 변경 불가
            // 숫자 배열 등에도 활용 가능
                let a = [1, 2, 3, 4, 5];
                let slice = &a[1..3];
                assert_eq!(slice, &[2, 3]);
                println!("슬라이스 타입 - Success!");
                


}

fn another_function_1() {
    println!("함수 동작 원리 - another_function_1");
}

fn another_function_2(x: i32, y: i32) {
    println!("함수 동작 원리 - another_function_2: The value of x is {}", x);
}

fn another_function_3(x: i32, y: i32) -> i32 {
    x + y
}

fn another_function_4(x: i32) -> i32 {
    if x == 0 {
        return -1;
    }
    x
}

fn sum_all_1(list: [i32; 5]) -> i32 {
    let mut result = 0;
    for i in list {
        result += i;
    }
    result
}

fn sum_all_2(list: [i32; 5]) -> i32 {
    list.iter().sum()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn print_str_1(s: String) -> String {
    println!("{}", s);
    s
}

fn print_str_2(s: String) {
    println!("{}", s);
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    // 복습하면서 보충 필요
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
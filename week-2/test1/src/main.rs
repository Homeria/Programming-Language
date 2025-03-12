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



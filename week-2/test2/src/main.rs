fn main() {
    
    // Struct(구조체)
        // 여러 값을 묶어서 사용하는 데이터 타입
        // 기본적으로 구조체 인스턴스는 바꿀 수 없음
        let user = User {
            username: String::from("차경호"),
            email: String::from("ckrudgh77@kangwon.ac.kr"),
            sign_in_count: 1,
            active: true
        };

        // 구조체 인스턴스 내부 값 변경
            // 특정 필드만 가변성 부여 불가능
            let mut user2 = User {
                username: String::from("차경호"),
                email: String::from("ckrudgh77@kangwon.ac.kr"),
                sign_in_count: 1,
                active: true
            };
            user2.username = String::from("차경호2");
        
        // 구조체 내부 값 접근
            // ".필드이름"으로 필드에 접근
            // ".index"로 접근하는 tuple과 비슷
            print!("{}", user.username);
        
        // 빠른 필드 값 설정
            // 함수 파라미터와 필드의 이름이 일치하는 경우 생략 가능
            let user3 = build_user(user.email, user.username);

        // 기존 구조체 인스턴스 값 사용
            // 명시된 필드를 제외하고 기존 인스턴스 값 사용
            // "=" 연산과 동일, 소유권 또한 넘겨짐짐
            let user4 = User {
                username: String::from("홍길동"),
                ..user3
            };
        
        // Tuple 구조체
            // 필드의 이름이 없는 구조체
            let black = Color(0, 0, 0);
            let origin = Point(0, 0, 0);

            // ex) 사각형의 넓이를 계산하는 코드드
                let rect1 = (30, 50);
                println!(
                    "The area of the rectangle is {} square pixels.",
                    area(rect1)
                );
        
        // 필드가 없는 구조체
            let subject = AlwaysEqual;


        // Debug Trait
            // {:?}을 사용하여 디버그 형식으로 출력
            // 함수 위 [#[derive(Debug)]]를 추가하여 사용
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            println!("rect1 is {:#?}", rect1);

        // dbg! 사용하기
            // 코드 동작 분석에 유용
            let scale = 2;
            let rect1 = Rectangle {
                width: dbg!(30 * scale),
                height: 50,
            };
            println!("rect1 is {:#?}", rect1);

        // 구조체 함수
            // impl을 사용하여 타입에 함수 구현
            let rect2 = Rectangle2 {
                width: 30,
                height: 50,
            };
            // println!{
            //     "The area of the rectangle is {} square pixels.",
            //     rect2.area()
            // }
        
        // 구조체 실습
        
            let p = build_person(String::from("example"), 20);
            println!("{:#?}", p);

            let viking = Viking::new("name", "country");
            println!("{:#?}", viking);
            


}



struct User {
    // 요소 이름 : 요소 타입
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 필드의 이름이 없는 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 필드가 없는 구조체
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Viking {
    name: String,
    country: String,
}

fn build_user(email: String, username: String) -> User {
    // email과 username은 parameter의 변수 이름과 같으므로 구조체 선언 시 생략 가능능
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_person(name: String, age: u32) -> Person {
    Person {
        name,
        age,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Self {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

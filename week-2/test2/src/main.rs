/**
 * 구조체의 예시를 위한 구조체
 * practice_struct_1() 함수에서 사용
 * username: 사용자 이름
 * email: 사용자 이메일
 * sign_in_count: 로그인 횟수
 * active: 활성화 여부
 * @use practice_sturct_1()
 */
struct User {
    // 요소 이름 : 요소 타입
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/**
 * Parameter로 구조체인 user를 생성하기 위한 요소들을 받아 User 구조체를 반환하는 함수
 * Parameter로 받은 email과 username은 필드의 이름과 일치하여 생략 가능
 * @param email: String
 * @param username: String
 * @return User
 * @use practice_sturct_1()
 */
fn build_user(email: String, username: String) -> User {
    // email과 username은 parameter의 변수 이름과 같으므로 구조체 선언 시 생략 가능능
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 필드의 이름이 없는 구조체

/**
 * RGB 값을 나타내는 튜플 구조체
 * @use practice_sturct_1()
 */
struct Color(i32, i32, i32);

/**
 * 3차원 좌표를 나타내는 튜플 구조체
 * @use practice_sturct_1()
 */
struct Point(i32, i32, i32);

// 필드가 없는 구조체

/**
 * ??
 * @use practice_sturct_1()
 */
struct AlwaysEqual;

/**
 * Parameter로 받은 width와 height를 이용하여 사각형의 넓이를 반환하는 함수
 * @param width: u32
 * @param height: u32
 * @return u32
 * @use practice_sturct_1()
 */
fn get_area_with_width_and_height(width: u32, height: u32) -> u32 {
    width * height
}

/**
 * Parameter로 받은 dimensions(튜플)을 이용하여 사각형의 넓이를 반환하는 함수
 * @param dimensions: (u32, u32)
 * @return u32
 * @use practice_sturct_1()
 */
fn get_area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/**
 * 사각형을 구성하는 구조체
 * @use practice_sturct_1()
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/**
 * 사각형인 Rectangle 구조체에 대한 함수
 */
impl Rectangle {
    /**
     * 사각형의 넓이를 반환하는 함수
     * @return u32
     * @use practice_sturct_1()
     */
    fn area(&self) -> u32 {
        // &self는 self: &Self를 의미
        // Self는 impl의 대상이 되는 타입
        self.width * self.height
    }
}

/**
 * 구조체의 대한 기본 구조
 * User 구조체를 이용하여 예시를 작성
 * @use main()
 */
fn practice_struct_1() {

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
            // 특정 필드만 가변성 부여 불가능 -> 모든 인스턴스에 mut 부여여
        let mut mutable_user = User {
            username: String::from("차경호"),
            email: String::from("ckrudgh77@kangwon.ac.kr"),
            sign_in_count: 1,
            active: true
        };
        mutable_user.username = String::from("차경훈");
        
        // 구조체 내부 값 접근
            // ".필드이름"으로 필드에 접근
            // ".index"로 접근하는 tuple과 비슷
            print!("{}", user.username);
        
        // 빠른 필드 값 설정
            // 함수 파라미터와 필드의 이름이 일치하는 경우 생략 가능
            let user2 = build_user(user.email, user.username);

        // 기존 구조체 인스턴스 값 사용
            // 명시된 필드를 제외하고 기존 인스턴스 값 사용
            // "=" 연산과 동일, 소유권 또한 넘겨짐 -> user2는 더 이상 소유권이 없어짐
            let user3 = User {
                username: String::from("홍길동"),
                ..user2
            };
        
        // Tuple 구조체
            // 필드의 이름이 없는 구조체
            let black = Color(0, 0, 0); // Black의 RGB 값은 (0, 0, 0)
            let origin = Point(0, 0, 0); // 3차원 공간에서의 원정의 좌표는 (0, 0, 0)

            // ex) 사각형의 넓이를 계산하는 코드
                
                // Parameter로 width와 height를 직접 주는 경우
                let area1 = get_area_with_width_and_height(30, 50);

                // Parameter로 width와 height가 있는 튜플로 주는 경우
                let rect1 = (30, 50);
                let area2 = get_area_with_tuple(rect1);
                println!(
                    "The area of the rectangle is {}, {} square pixels.",
                    area1,
                    area2
                );
        
        // 필드가 없는 구조체
            let subject = AlwaysEqual;


        // Debug Trait
            // {:?}을 사용하여 디버그 형식으로 출력
            // 구조체 위 [#[derive(Debug)]]를 추가하여 사용
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

fn main() {
    
    practice_struct_1()
            
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



fn build_person(name: String, age: u32) -> Person {
    Person {
        name,
        age,
    }
}



impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Self {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

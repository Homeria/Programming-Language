// 라이브러리 불러오기
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// 함수 정의의
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    // 반복문 - 앞뒤로 loop만 붙히면 됨.
    // 종료하고 싶으면 조건문에 break
    loop {

        println!("Please input your guess.");

        // 변수 정의
        // let 키워드 사용
        // 기본적으로 변수 값 수정 불가
        // mut 키워드 추가하면 수정 가능
        let mut guess = String::new();

        // 사용자로부터 문자열을 입력받아 guess 문자열에 추가. 덮어쓰기 X
        // '&'는 해당 인수가 참조임을 의미
        // 변수가 여러 번 사용되는 경우 데이터를 메모리에 여러 번 복사하지 않고
        // 동일한 데이터에 엑세스할 수 있는 기능
        // 참조도 기본은 불변. 데이터 변경 필요 시 '&guess' -> '&mut guess'
        io::stdin()
            .read_line(&mut guess)

            // read_line()을 실행한 경우 Result 반환
            // Result의 결과가 Err인 경우 expect()에 포함된 메세지 출력 후 종료
            // expect()가 없는 경우 아래와 같은 경고 출력
            .expect("Failed to read line");

        // 32비트 unsigned int로 파싱 -> 실패 시 오류 출력
        // 변수 유형이 반드시 입력되어야 함
        // parse()는 REsult 열거형 변환 (Ok, Err) 숫자 비교와 동일하게 match문 사용
        // 정상 처리된 경우 숫자 반환, 실패한 경우 continue로 loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,   
        };


        // {변수} 입력 가능. 빈 칸을 두고 쉼표로 구분하여 출력 가능 
        println!("You guessed : {guess}");

        // 랜덤으로 생성된 값(secret_number)과 비교
        // 패턴 -> 실행될 코드
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}

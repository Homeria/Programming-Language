pub mod problems {
    /// 1에서 n까지의 정수의 합을 구하는 함수 sum을 작성하라
    /// n ≤ 0인 경우 0을, n > 0인 경우 n(n+1)/2를 반환한다
    ///
    /// ```
    /// use rust_hw1::problems::sum;
    /// 
    /// assert_eq!(sum(-10), 0);
    /// assert_eq!(sum(100), 5050);
    /// ```
    pub fn sum(n: i32) -> i32 {
        if n >= 0 {
            // n이 0보다 크거나 같을 경우
            ( n * (n + 1) ) / 2
        } else {
            // n이 0보다 작을 경우
            0
        }
    }

    /// 1에서 n까지의 정수의 곱을 구하는 함수 factorial을 작성하라
    /// n <= 0인 경우 0을 반환한다.
    ///
    /// ```
    /// use rust_hw1::problems::factorial;
    ///
    /// assert_eq!(factorial(-10), 0);
    /// assert_eq!(factorial(5), 120);
    /// ```
    pub fn factorial(n: i32) -> i32 {
        if n <= 0 {
            0
        } else {
            // factorial을 구하기 위한 mut 키워드를 넣은 변수 선언
            let mut result = 1;

            // 1. for문을 이용한 방법 - for .. in ..
            // 1..n은 1부터 n-1까지, 1..=n은 1부터 n까지
            for i in 1..=n {
                result *= i;
            }

            // 2. while 문을 이용한 방법
            /*
            cnt는 n부터 1까지 감소하며, 0이 되면 while문은 종료되어 result를 반환환
            let mut cnt = n;
            while cnt > 0 {
                result *= cntl
                cnt -= 1;
            }
            */

            result
        }
    }

    /// 반지름이 r인 원의 넓이를 구하는 함수 circle을 작성하라
    /// r ≤ 0.0인 경우 0.0을, r > 0.0인 경우 3.14 * r * r을 반환한다
    ///
    ///  ```
    /// use rust_hw1::problems::circle;
    ///
    /// assert_eq!(circle(-10.1), 0.0);
    /// assert_eq!(circle(4.2), 55.389595);
    /// ```
    pub fn circle(r: f32) -> f32 {
        if r <= 0.0 {
            // r이 0보다 작거나 같을 경우
            0.0
        } else {
            // r이 0보다 클 경우
            3.14 * r * r
        }
    }

    /// 문자열의 앞에 "Hello "를 삽입하는 함수 concat을 작성하라
    ///
    /// concat s는 문자열 s의 앞에 "Hello "를 삽입한 문자열을 반환한다. (Hello 뒤에 공백문자가 있음에 유의하라.)
    ///
    /// ```
    /// use rust_hw1::problems::concat;
    /// 
    /// assert_eq!(concat("Bob!"), "Hello Bob!");
    /// assert_eq!(concat("Alice!"), "Hello Alice!");
    /// ```
    pub fn concat(str: &str) -> String {
        /*
        문자열 생성 방법
        1. let mut s = String::new();
        2. let s = "initial contents".to_string();
        3. let s = String::from("initial contents");
         */
        let mut s = String::from("Hello ");

        /*
        문자열 업데이트
        1. 단어 붙이기 : s.push_str("abc");
        2. 글자 붙이기 : s.push('a');
         */
        s.push_str(str);
        s
    }

    /// 논리연산자 xor를 계산하는 함수 xor를 작성하라
    /// 
    /// 불린형 값 x와 y중 하나만 true일 경우에 true를, 이외의 경우 false를 반환한다
    /// 
    /// ```
    /// use rust_hw1::problems::xor;
    /// 
    /// assert_eq!(xor(true, true), false);
    /// assert_eq!(xor(true, false), true);
    /// assert_eq!(xor(false, true), true);
    /// assert_eq!(xor(false, false), false);
    /// ```
    pub fn xor(x: bool, y: bool) -> bool {
        if (x && y) || (!x && !y) {
            // x와 y가 모두 true이거나 둘 다 false인 경우 false를 반환
            false
        } else {
            // x와 y 둘 중 하나가 true, 나머지 하나가 false인 경우 true를 반환환
            true
        }
    }

    /// 리스트에서 가장 큰 값을 찾아서 반환하는 함수 max를 작성하라.
    /// 
    /// 5개의 값이 포함된 리스트에서 가장 큰 값을 찾아서 반환한다.
    /// 
    /// ```
    /// use rust_hw1::problems::max;
    /// 
    /// assert_eq!(max([1, 2, 3, 4, 5]), 5);
    /// assert_eq!(max([5, 4, 3, 2, 1]), 5);
    /// ```
    pub fn max(list: [u32; 5]) -> u32 {
        // 리스트의 첫 번째 원소를 max로 설정
        // 이는 max에 list[0]의 값을 복사한 것것
        let mut max = list[0];

        // 리스트를 iter 하며 if 하며 max를 찾음
        for &i in list.iter() {
            // i도 max와 마찬가지로 주소를 받은 것 -> 리스트 안 값은 &i를 사용
            if i > max {
                max = i;
            }
        }
        max
    }

    /// 세 정수를 변의 길이로 가지는 삼각형이 존재하는지 확인하는 함수 triangle을 작성하라.
    /// 
    /// x, y, z중 하나라도 0 또는 음수인 경우 false를,
    /// x, y, z가 모두 양수인 경우 x, y, z를 세 변으로 가지는 삼각형이 존재하면 true를, 존재하지 않으면 false를 반환한다.
    /// 
    /// ```
    /// use rust_hw1::problems::triangle;
    /// 
    /// assert_eq!(triangle(-3, 3, 1), false);
    /// assert_eq!(triangle(3, 4, 5), true);
    /// assert_eq!(triangle(100, 1, 2), false);
    /// ```
    pub fn triangle(x: i32, y: i32, z: i32) -> bool {
        false
    }

    /// 두 정수의 합과 차 중 하나를 선택하는 함수 if_else를 작성하라.
    /// 
    /// b가 true이면 x + y를, false이면 x − y를 반환한다.
    /// 
    /// ```
    /// use rust_hw1::problems::if_else;
    /// 
    /// assert_eq!(if_else(true, 2, 100), 102);
    /// assert_eq!(if_else(100 < 2, 2, -2), 4);
    /// ```
    pub fn if_else(b: bool, x: i32, y: i32) -> i32 {
        -1
    }

    /// 사칙연산을 구현하는 함수 calculator를 작성하라.
    /// 
    /// "+", "-", "*", "/" 연산을 수행하는 계산기를 구현한다.
    /// 다른 연산자가 들어오는 경우 –1.0을 반환한다.
    /// 
    /// ```
    /// use rust_hw1::problems::calculator;
    /// 
    /// assert_eq!(calculator(1.0, "+", 3.0), 4.0);
    /// assert_eq!(calculator(3.0, "-", 1.0), 2.0);
    /// assert_eq!(calculator(2.0, "*", 4.0), 8.0);
    /// assert_eq!(calculator(3.0, "/", 1.5), 2.0);
    /// ```
    pub fn calculator(x: f32, operator: &str, y: f32) -> f32 {
        -1.0
    }

    pub struct Point {
        pub x: f32,
        pub y: f32
    }

    /// 점 사이의 거리를 계산하는 함수 distance를 작성하라.
    /// 
    /// ((a.x - b.x)^2 + (a.y - b.y)^2)의 제곱근을 계산하여 반환한다.
    /// 
    /// ```
    /// use rust_hw1::problems::{ Point, distance };
    /// 
    /// let a = Point { x: 1.0, y: 2.0 };
    /// let b = Point { x: 4.0, y: 6.0 };
    /// assert_eq!(distance(a, b), 5.0);
    /// ```
    pub fn distance(a: Point, b: Point) -> f32 {
        -1.0
    }
}

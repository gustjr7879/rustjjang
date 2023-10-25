extern crate rand; 
use std::io; //std라이브러리에 io라이브러리 import
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secretnum = rand::thread_rng().gen_range(1,101);
    println!("Secret Number is {}",secretnum);
    loop {
    println!("Please input your guess.");
    let mut guess = String::new(); //guess mutable(가변)
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed {}", guess);
    match guess.cmp(&secretnum){
        Ordering::Less  => println!("Too small!"),
        Ordering::Equal => {
            println!("Correct!");
            break;                }
        Ordering::Greater => println!("Too big!"),
    }
}
}
//사용자가 추리한 값을 입력받아서 그대로 출력하는 코드 
//사용자의 입력을 받고 결과값을 표시하기위해서는 io(input/output)라이브러리가 필요함 io라이브러리는 std라고 불리는 표준 라이브러리에 있음
//변수를 생성하는 let문 러스트에서 기본적으로 변수는 불변 ! mut을 붙여주면 가변 변수가 된다 !
//String은 문자열 타입이고, ::new()는 new가 String타입의 연관함수임을 나타낸다. 연관함수란 하나의 타입을 위한 함수
//즉 6번줄은 새로운(new) 빈 String타입의 인스턴스가 가변 guess변수이다 라는 것임
//io의 연관함수 stdin() 불러옴 stdin의 read_line메소드로 입력을 받는것이고, 이를 guess에 지정해줌
//&는 코드의 여러 부분에서 데이터를 여러번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자임을 나타냄
//참조자는 기본적으로 불변이라서 mut으로 만들어줌 
//io::stdin().read_line 은 guess에 값을 저장해준 후 io::Result 인스턴스를 반환해주는데, 이때 에러가 나면 expect에 있는 코드를 진행해준다.
//어떠한 패키지를 다운받아야하는 경우 : cargo.toml에 depen... 밑에다가 패키지 이름과 버전을 적어놓고 cargo build하면 다운한다.
//extern crate rand;를 통해서 외부에 의존하는 크레이트 있음을 알림
//rand::thread_rng 함수는 OS가 시드를 정하고 현재 스레드에서만 사용되는 정수를 gen_range(1,10) 내에서 생성해줌
//cmp 메소드는 readline과 같게 Result를 뱉어주고 비교할 수 있게한다. 
fn main() {
    println!("Hello, world!");
    another_function(); // 함수 이름과 ()를 통해서 다른 함수를 호출할 수 있다.
    another_function1(5);
    another_function2(5,6);
    another_function3();
    let x = five();
    println!("The value of x {}",x);
    let x = plus_one(x);
    println!("The value of x {}",x)
}
fn another_function(){
    println!("Another function working !");
}
fn another_function1(x:i32){
    println!("The value of x {}",x);
}
fn another_function2(x:i32,y:i32){
    println!("The value of x {}",x);
    println!("The value of y {}",y);
}
fn another_function3(){
    let x = 5;
    let y = {
        let x = 3; //여기서만 사용되는 x
        x+1
    };
    println!("The value of y is {}",y);
    println!("The value of x is {}",x); //x는 5로 유지되고 있음
}
fn five() -> i32 {
    5
}
fn plus_one(x:i32) -> i32{
    x + 1
}
// rust는 main함수가 가장 중요하다. 이 함수는 다수의 프로그램에서 실행 지점이다. fn을 사용해서 새로운 함수를 선언할 수 있다. 
// rust는 변수나 함수이름은 모두 소문자로 하고 밑줄 표시로 단어를 구분하는 snake를 사용한다.
// 또한 함수를 선언할 때 ()안에 매개변수를 받게 할 수 있고 타입또한 지정해줄 수 있다. 반드시 타입을 명시해서 다른사람이 봤을 때 의도를 고민하게 만들지 말자.
// rust는 다른 함수와 다르게 x = y = 5 와 같은 코드를 이해하지 못한다. 단순 선언만 한것이지, 산출하지 않았기 때문이다. 따라서 매크로라는 것을 호출해서 산출해줘야한다.
// 사용 방법은 {}을 통해서 해준다. 참고로 이런 산출하는 표현식은 세미콜론을 사용하지 않는다. 세미콜론을 사용하면 구문으로 변경되고 반환하지 않기 때문이다.
// 더 자세하게 보면 five함수에서 i32타입으로 함수를 {}을 통해서 반환해주고 main 함수에서는 변수 x가 five의 반환값이라고 한다. 따라서 x는 5로 출력이 된다. 즉 함수를 선언할 때 ->는 -> 뒤에 있는 타입으로 반환 하겠다. 이말이다.
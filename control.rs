fn main() {
    let num = 3;
    if num < 5 {
        println!("True");
    }
    else {
        println!("False");
    }
    ifprac(num);
    let condition = true;
    let num2 = if condition {
        5
    } else {
        6
    };
    println!("The value of num2 {}",num2);
    let mut number = 3;
    while number != 0{
        println!("{}!",number);
        number = number -1;
    }
    println!("Loop OFF!!!!");

    let a = [1,2,3,4,5];
    let mut index = 0;
    while index <5 {
        println!("The value is : {}",a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("for - The value is : {}",element);
    }
    for n in 1..4{
        println!("for 1-4 The value is : {}",n)
    }
}
// 제어문 (조건문)은 익숙하지만 문법이 살짝 다르다. if 조건 {} else {} 으로 진행되며 안에 코드에서 실행되는 것을 말해줘야한다. else if 를 통해서 추가적인 조건 설정도 가능하다. 
// let으로 변수를 선언해주면서 if문을 섞어서 쓸 수 있다. 다만 다른 점은 변수 선언이니 if문을 사용하고 ;으로 끝맺음을 해줘야한다.
// 제어문 if에 나오는 값 타입은 호환되지 않는다. 즉 let num2 = if condition {5} else{'six'}와 같이 다른 식으로 표현되면 오류가 발생한다. 왜냐하면 타입을 알아야지 모든 곳에서 유효한지 판단할 수 있기 때문.
// while 을 저렇게 사용해서 index를 출력할 수 있지만, 이 경우엔 한번씩 조건을 체크하는 번거로움이 발생하고 잘못하면 패닉이 발생할 수 있다.
// 따라서 인덱스를 사용하는 용도로는 for문을 사용할 수 있으며 버그를 자제할 수 있다. a.iter는 a가 돌수있는 만큼이 된다.
// range기능은 숫자 .. 숫자로 사용 가능하고 만약에 역으로 돌리고 싶거나 어떤 기능을 원한다면 ()로 숫자들을 묶어준 후 .rev()나 다른 메서드를 사용해서 돌릴 수 있다. 
fn ifprac(x:i32){
    if x % 4 ==0{
        println!("number is divisible by 4");
    }
    else if x % 5 == 0{
        println!("number is divisible by 5");
    }
    else if x % 2 == 0{
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 2,4,5");
    }
}

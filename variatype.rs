fn main() {
    let x = 2.0; //f64로 설정
    let y: f32 = 3.0;//f32로 설정
    println!("Type are {}, {}",x,y);
    let t = true; //이래도 boolean
    let f: bool = false; //이렇게 타입을 설정해줄 수 있다.
    println!("Boolean are {},{}",t,f);
    let c = 'z';
    let z = 'Z';
    println!("char는 ''이다. {},{}",c,z);
    let tup: (i32,f64,u8) = (500,6.4,1);
    let tup = (500, 6.4,1);
    let (nx,ny,nz) = tup;
    println!("The value of ny is {}",ny);
    let x:(i32,f64,u8) = (500,6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tuple index {},{},{}",five_hundred,six_point_four,one);
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("Array index {},{}",first,second)
}
//rust의 모든 값들은 타입을 가지고, 러스트는 타입이 고정된 언어이다. 즉 변수의 타입이 컴파일 시 반드시 정해져있어야한다.
//Scalar는 하나의 값으로 표현되는 타입임. 정수형 부동소수점 숫자, boolean, 문자 네가지 스칼라타입을 보유하고 있음
//정수형 -> u8 u16 u32 u64 usize 부호가 없는 타입은 u로 시작하고 부호가 있는 타입은 i로 시작한다. size는 아키텍처(32비트 64비트)에 따라서 결정된다. 보통은 i32가 가장 빠르고 크니 이를 사용하자
//연산자는 패스
//Boolean타입 -> true/false
//rust에서 string은 ""을 사용하지만 char 타입은 ''를 사용함
//복합타임들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있다. Rust는 튜플과 배열이 존재한다. 괄호안에 콤마로 구분되는 값들을 작성함 포함되는 값들의 타입은 달라도됨
// let tup = (500,6.4,1); 을 통해서 tup에 튜플 저장하고, let (nx,ny,nz) = tup; 을 통해서 nx ny nz을 한번에 선언할 수 있음.
// 또한 마침표 뒤에 우리가 접근하길 원하는 값의 색인을 넣는것을 통해서 튜플의 요소에 직접적으로 접근할 수 있음. 즉 튜플의 인덱스로 접근하려면 .0 .1 등과 같이 점을 사용해서 할 수 있다.
// 여러 값들의 집합체를 만드는 것에는 배열 또한 있는데, 튜플과 다르게 모두 동일한 타입이여야한다. 다른 언어들과 rust 배열의 다른 점은 고정된 길이를 갖는다는 점이다. 한번 선언되면 크기가 작아지거나 커지지 않는다.
// heap보다 stack에 할당하는 것을 원하거나 항상 고정된 요소를 가진다고 확신할 때 사용한다. 벡터타입처럼 가변성이 존재하지 않는다. 만약에 변할것같다 싶으면 벡터타입을 사용하는게 맞다.
// 튜플은 .으로 index하는데 배열은 파이썬과 똑같이 한다. 
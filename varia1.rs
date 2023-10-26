fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6; // 이 시점에 mut 소실됨
    println!("The mut value of x is {}",x);
    const MAX_POINT: u32 = 100_000;
    println!("The const value of MAX_POINT is {}",MAX_POINT);
    let x = x +1; //shadowing으로 불변성 해소시킴
    let x = x*2;
    println!("The shadowing value of x is {}",x);
    let spaces = "    ";
    let spaces = spaces.len(); //str에서 usize로 타입을 변경하면서 같은 변수 이름으로 끌고올 수 있음
    println!("The shadowing type change {}",spaces)
}
// rust언어는 기본적으로 변수가 불변형으로 들어간다. 그러기 때문에 mut을 사용해서 가변으로 안만들어주면 오류가 발생하게 된다. 
// 변수는 가변 불변으로 설정할 수 있지만, 만약에 상수(안변하는 값)라면 let 대신 const 키워드를 사용해서 값을 선언할 수 있다. (rust의 상수 명명 규칙에 따라서 모든 단어를 대문자로함)
// 변하지 않는 값이므로 프로그램 전체적으로 통용되는 값을 지정할 때 편리하며 유지보수할 때도 편리하다
// rust에는 shadowing 기능이 존재한다. shadowing이란 기존에 존재하던 변수의 이름과 같은 이름으로 새로운 변수를 선언할 수 있고, 이 경우에 이전에 변수를 shadow 즉 따라한다.
// shadowing하면 mut으로 선언하는 것과 차이가 있다. 가변 변수에 let 키워드를 사용하지 않고 변수에 새로운 값을 대입하려고 하면 한번 이후부터는 불변으로 설정되어서 컴파일 시 오류가 발생한다. 
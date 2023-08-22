fn main() {
    let x = 5; //불변이라서 변경할 수 없음
    println!("The value of x is {}",x);
    let mut y = 5;
    println!("The value of y is {}",y);
    y = 7; //가변이라서 변경가능
    println!("The value of y is {}",y);
    //하지만 이것들은 변수 (let을 사용함)이기 때문에 변경가능하게 조정할 수 있는것이고 상수라면 아예 변경하지 못함(const)
    const MAX_POINTS: u32 = 100_000;
    println!("{}",MAX_POINTS);
    //Shadowing 개념이란 새 변수가 이전 변수를 shadow하고 차지한다.
    let x = x + 1; //이렇게 하면 새로운 변수 x가 이전 변수 x에 1을 더한것과 같다.
    println!("new x is {}",x);
    // mut과 차이가 있는데 mut으로 변수설정 후 값을 변경하게 된다면 let없이 변경가능하지만 shadow로 값을 변경하려면 let을 사용해줘야한다. 또 다른 차이점은 동일 이름을 사용하면서 값의 유형을 변경할 수 있다는 점이 있다
    let spaces = "     ";
    println!("spaces is {}",spaces);
    let spaces = spaces.len();
    println!("spaces is {}",spaces);
}

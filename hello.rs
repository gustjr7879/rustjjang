fn main() {
    println!("Hello, world!");
}
//주석은 이렇게 달 수 있음
//컴파일 방법 : rustc hello.rs -> ./hello 파일 생성 -> ./hello 입력 시 파일 실행
//main함수는 C패밀리 언어들에서 많이 사용되며 프로그램 내에서 가장 먼저 실행되는 함수임
//()안에 파라미터 {}안에 본체가 들어감
//println! 은 러스트 매크로(macro)라고 함 함수로 쓸려면 !없이 사용해야함
//Cargo는 러스트의 빌드시스템 및 패키지 매니저임 코드를 빌드하고, 필요한(의존하는) 라이브러리들을 다운로드해줌
//cargo new hello_cargo --bin 하게되면 cargo가 hello_cargo 폴더를 만들고, 파일을 추가해준다(기본적인)
//src 디렉터리에 rust파일이 생성되어있고, Cargo.toml에 버전 등의 정보가 담긴다.
//cargo를 동해서 빌드하는 방법은 폴더로 이동 후 cargo build 커맨드를 사용해서 빌드할 수 있다.
//빌드하게되면 target/debug에 파일이 생성된다. 또한 이 과정을 한번에 묶어서 cargo run 커맨드를 쓸 수 있다.
//코드를 짜다가 뭔가 불안하다 싶으면 cargo check를 통해서 컴파일이 되는지 확인할 수 있다.
//즉 cargo check : 확인 cargo build : 실제 컴파일 cargo run : 컴파일부터 실행파일까지

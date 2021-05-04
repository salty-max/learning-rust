fn main() {
    // Mutable string
    // let mut s = String::new();

    let data = "initial content";
    let _s = data.to_string();

    // The above is equivalent to
    // let s = "initial content".to_string();
    // Or
    // let s = String::from("initial content");

    // UTF-8 encoded
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    push_to_string();
    plus_operator();
    with_format();

    let _hello = String::from("Hello");
    // let h = hello[0];   Can not index String with integer

    iter_over_string();
}

fn push_to_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s3 = String::from("LO");
    s3.push('L');
}

fn plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    println!("s3 is {}", s3);
}

fn with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 👎
    // let s = s1 + "-" + &s2 + "-" + &s3;

    // 👍
    // Bonus: format! doen't take onwership of any parameter
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}

fn iter_over_string() {
    let hello = String::from("こんにちは、ニュー");
    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}

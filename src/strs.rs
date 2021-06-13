pub fn run() {
    println!("{} from {}" ,"hello", "print");
    println!("{1} from {2} and {0} + {0} ," , "hello" , "print" , "rust");
    
    // string type
    let x = String::from("hello");
    println!("{}", x);
    // get length
    println!("{}", x.len());
    // push element
    let mut pushele = String:: from("hello");
    // push char
    pushele.push('w');
    // push str
    pushele.push_str("world");
    println!("{}", pushele);
    // capactiy 
    println!("{}",pushele.capacity());
    // is empty
    println!("{}", pushele.is_empty());
    // containes 
    println!("{}", pushele.contains('h'));
    // replace 
    println!("{}", pushele.replace("hello", "world"));
    // loop 
    for word in pushele.split_whitespace() {
        println!("{}", word)
    }
    // str with capactiy
    let mut s = String::with_capacity(10);
    s.push('q');
    s.push('w');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);

}
fn strings(){
    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str("word");

    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}
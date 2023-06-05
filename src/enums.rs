fn enumerators(){
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates monday"),
        Day::Tuesday => println!("Everyone hates tuesday"),
        Day::Wednesday => println!("Everyone hates wednesday"),
        Day::Thursday => println!("Everyone hates thursday"),
        Day::Friday => println!("Everyone hates friday"),
        Day::Saturday => println!("Everyone hates saturday"),
        Day::Sunday => println!("Everyone hates sunday"),
    }

    println("Is today weekedn {}", today.is_weekend())
}
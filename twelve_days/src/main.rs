fn main() {
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let other_gifts = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut day = 1;
    while day <= 12 {
        print!("On the ");
        print!("{}", ordinals[day - 1]);
        println!(" day of Christmas");
        println!("My true love gave to me");
        if day == 1 {
            print!("A ");
        } else {
            let mut current_day = day;
            while current_day >= 2 {
                println!("{}", other_gifts[current_day - 2]);
                current_day -= 1;
            }
            print!("And a ");
        }
        println!("partridge in a pear tree.\n");
        day += 1;
    }
}

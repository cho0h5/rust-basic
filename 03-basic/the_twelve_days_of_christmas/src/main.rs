fn main() {
    let first2twelfth = ["first", "seconde", "third", "fourth",
                         "fifth", "sixth", "seventh", "eighth",
                         "ninth", "tenth", "eleventh", "twelfth"];

    let bonus = ["And a patridge in a pear tree",
                 "Two turtle doves",
                 "Three French hens",
                 "Four calling birds",
                 "Five gold rings, badam-pam-pam",
                 "Six geese a laying",
                 "Seven swans a swimming",
                 "Eighth maids a milking",
                 "Nine ladies danceing",
                 "Ten lords a leaping",
                 "Eleven pipers piping",
                 "12 drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas", first2twelfth[i]);
        println!("My true love gave to me");
        for j in (0..i+1).rev() {
            println!("{}", bonus[j]);
        }
        println!();
    }
}

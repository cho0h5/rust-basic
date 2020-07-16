fn main() {
    //let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 4, 5];

    v2.push(100);
    v2.push(-1);

    println!("{:?}", v2);

    //let third: &i32 = &v2[2];
    //let third: Option<&i32> = v2.get(2);
    
    for i in &mut v2 {
        *i += 1;
        println!("{}", i);
    }

    println!("{:?}", v2);


    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("asjdfhweh")),
        SpreadsheetCell::Float(2.9147547),
        SpreadsheetCell::Int(222),
    ];

    println!("{:?}", row);
}

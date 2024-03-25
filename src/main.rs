use std::io;


use crate::entity::message::Message;

pub mod entity;


fn main() {
    //  println!("input a number");

    //  let mut user_input = String::new();

    //  io::stdin().read_line(&mut user_input).expect("failed to read line");

    //  let mut num = user_input.trim().parse().expect("is not a number");

    //  num = double_num(num);

    //  let user_input2= String::from("1234");

    //  let user_num = transStr(&user_input2);

    //  println!("{user_input2}, {user_num}");

    //  println!("you input is {num}");

    //  let mut  str1 = "this is str";

    //  str1 = "this is str1234";

    //  println!("{str1}")

    // let entity = Entity {
    //     name: String::from("123"),
    // };

    // let name = entity.get_name();

    // println!("name is {}",name);

    // dbg!(&entity);


    let quit_message = Message::Quit;

    let write_message = Message::Write(String::from("hello"));

    // dbg!(quitMessage);

    // dbg!(writeMessage);

    quit_message.call();

    write_message.call();


}

// fn double_num(input: isize) -> isize {
//     return input * 2;
// }

// fn transStr(input: &String) -> i64 {
//     return input.trim().parse().expect("not a number");
// }

// #![allow(dead_code)]
// // panic
// fn give_princess(gift: &str) {
//     if gift == "snake" {
//         panic!("AAAAaaaaa!!!");
//     }
//     println!("I love {}s!!!!!", gift);
// }
//
// // fn main() {
// //     give_princess("teddy bear");
// //     give_princess("snake");
// // }
//
// fn give_commoner(gift: Option<&str>) {
//     match gift {
//         Some("snake") => println!("AAAAaaaaaaaaa!!!!"),
//         Some(inner) => println!("{}? How nice.", inner),
//         None => println!("No gift? Oh well."),
//     }
// }
//
// fn give_princess2(gift: Option<&str>) {
//     // unwrap在接收到None时将会返回panic
//     let inside = gift.unwrap();
//     if inside == "snake" {
//         panic!("AAAAAAAAAAAAAAAaaa!!!");
//     }
//
//     println!("I love {}s!!!!", inside);
// }
//
// // fn main() {
// //     let food = Some("chicken");
// //     let snake = Some("snake");
// //     let void = None;
// //
// //     give_commoner(food);
// //     give_commoner(snake);
// //     give_commoner(void);
// //
// //     let bird = Some("robin");
// //     // let nothing = None;
// //
// //     give_princess2(bird);
// //     // give_princess2(nothing);
// // }
//
// // 可以使用组合算子，以模块化的风格来管理控制流
//
// #[derive(Debug)]
// enum Food {
//     Apple,
//     Carrot,
//     Potato,
// }
//
// #[derive(Debug)]
// struct Peeled(Food);
// #[derive(Debug)]
// struct Chopped(Food);
// #[derive(Debug)]
// struct Cooked(Food);
//
// // 削皮，如果没有食物，就返回None，否则返回削好皮的食物
// fn peel(food: Option<Food>) -> Option<Peeled> {
//     match food {
//         Some(food) => Some(Peeled(food)),
//         None => None,
//     }
// }
//
// //切食物，如果没有食物，返回None，否则返回切好的食物
// fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
//     match peeled {
//         Some(Peeled(food)) => Some(Chopped(food)),
//         None => None,
//     }
// }
//
// // 烹饪食物，这里使用map()来替代match处理各种情况
// fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
//     chopped.map(|Chopped(food)| Cooked(food))
// }
//
// // 这个函数会完成削皮切块烹饪一条龙
// fn process(food: Option<Food>) -> Option<Cooked> {
//     food.map(|food| Peeled(food))
//         .map(|Peeled(food)| Chopped(food))
//         .map(|Chopped(food)| Cooked(food))
// }
//
// fn eat(food: Option<Cooked>) {
//     match food {
//         Some(food) => println!("Mmm. I love {:?}", food),
//         None => println!("Oh no! It wasn't edible."),
//     }
// }
//
// pub fn for_map() {
//     let apple = Some(Food::Apple);
//     let carrot = Some(Food::Carrot);
//     let potato = None;
//
//     let cooked_apple = cook(chop(peel(apple)));
//     let cooked_carrot = cook(chop(peel(carrot)));
//
//     let cooked_potato = process(potato);
//
//     eat(cooked_apple);
//     eat(cooked_carrot);
//     eat(cooked_potato);
// }
//
#![allow(dead_code)]

use std::num::ParseIntError;

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// 使用and_then
fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
        None => println!("Oh no, we don't get to eat on {:?}?", day),
    }
}

pub fn for_and_then() {
    let (cordon_blue, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_blue, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}

// Result是Option的类型更丰富的版本，描述的是可能的错误而不是可能的不存在
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

pub fn for_result() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);
    // 在失败情况下，parse产生一个错误，留给unwrap来解包并产生panic，另外，panic会退出我们的程序，来提供一个让人不爽的错误消息

    // 这样可以知道函数的返回类型详情
    // let i: () = "t".parse::<i32>();
}

// 使用简单的match语句导致更加繁琐的代码
fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn for_result_map_v1() {
    let twenty = multiply_v1("10", "2");
    print(twenty);

    // 这种情况下会提供一条更有用的错误信息
    let tt = multiply_v1("t", "2");
    print(tt);
}

// 幸运的是，Option的map、and_then、以及很多其他组合算子也为Result实现了
// 下面除了写法外，与上面那个完全一致
// 它的作用是：如果值是合法的，计算其乘积，否则返回错误
fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print_v2(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn for_result_map_v2() {
    let twenty = multiply_v2("10", "2");
    print_v2(twenty);

    // 这种情况下就会提供一条更有效的信息
    let tt = multiply_v2("t", "2");
    print_v2(tt);
}

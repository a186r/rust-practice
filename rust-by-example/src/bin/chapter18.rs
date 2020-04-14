#![allow(dead_code)]
// panic
fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAAaaaaa!!!");
    }
    println!("I love {}s!!!!!", gift);
}

// fn main() {
//     give_princess("teddy bear");
//     give_princess("snake");
// }

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("AAAAaaaaaaaaa!!!!"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess2(gift: Option<&str>) {
    // unwrap在接收到None时将会返回panic
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAAAAAAAAAAAAAaaa!!!");
    }

    println!("I love {}s!!!!", inside);
}

// fn main() {
//     let food = Some("chicken");
//     let snake = Some("snake");
//     let void = None;
//
//     give_commoner(food);
//     give_commoner(snake);
//     give_commoner(void);
//
//     let bird = Some("robin");
//     // let nothing = None;
//
//     give_princess2(bird);
//     // give_princess2(nothing);
// }

// 可以使用组合算子，以模块化的风格来管理控制流

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// 削皮，如果没有食物，就返回None，否则返回削好皮的食物
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

//切食物，如果没有食物，返回None，否则返回切好的食物
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// 烹饪食物，这里使用map()来替代match处理各种情况
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 这个函数会完成削皮切块烹饪一条龙
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|food| Peeled(food))
        .map(|Peeled(food)| Chopped(food))
        .map(|Chopped(food)| Cooked(food))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

pub fn for_map() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
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

use std::fmt::{Error, Formatter};
use std::intrinsics::write_bytes;
use std::num::ParseIntError;
use std::{error, fmt, result};

// #[derive(Debug)]
// enum Food {
//     CordonBleu,
//     Steak,
//     Sushi,
// }
//
// #[derive(Debug)]
// enum Day {
//     Monday,
//     Tuesday,
//     Wednesday,
// }
//
// fn have_ingredients(food: Food) -> Option<Food> {
//     match food {
//         Food::Sushi => None,
//         _ => Some(food),
//     }
// }
//
// fn have_recipe(food: Food) -> Option<Food> {
//     match food {
//         Food::CordonBleu => None,
//         _ => Some(food),
//     }
// }
//
// fn cookable_v1(food: Food) -> Option<Food> {
//     match have_ingredients(food) {
//         None => None,
//         Some(food) => match have_recipe(food) {
//             None => None,
//             Some(food) => Some(food),
//         },
//     }
// }
//
// // 使用and_then
// fn cookable_v2(food: Food) -> Option<Food> {
//     have_ingredients(food).and_then(have_recipe)
// }
//
// fn eat(food: Food, day: Day) {
//     match cookable_v2(food) {
//         Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
//         None => println!("Oh no, we don't get to eat on {:?}?", day),
//     }
// }
//
// pub fn for_and_then() {
//     let (cordon_blue, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
//
//     eat(cordon_blue, Day::Monday);
//     eat(steak, Day::Tuesday);
//     eat(sushi, Day::Wednesday);
// }
//
// // Result是Option的类型更丰富的版本，描述的是可能的错误而不是可能的不存在
// fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
//     let first_number = first_number_str.parse::<i32>().unwrap();
//     let second_number = second_number_str.parse::<i32>().unwrap();
//     first_number * second_number
// }
//
// pub fn for_result() {
//     let twenty = multiply("10", "2");
//     println!("double is {}", twenty);
//
//     // let tt = multiply("t", "2");
//     // println!("double is {}", tt);
//     // 在失败情况下，parse产生一个错误，留给unwrap来解包并产生panic，另外，panic会退出我们的程序，来提供一个让人不爽的错误消息
//
//     // 这样可以知道函数的返回类型详情
//     // let i: () = "t".parse::<i32>();
// }
//
// // 使用简单的match语句导致更加繁琐的代码
// fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     match first_number_str.parse::<i32>() {
//         Ok(first_number) => match second_number_str.parse::<i32>() {
//             Ok(second_number) => Ok(first_number * second_number),
//             Err(e) => Err(e),
//         },
//         Err(e) => Err(e),
//     }
// }
//
// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n) => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
//
// pub fn for_result_map_v1() {
//     let twenty = multiply_v1("10", "2");
//     print(twenty);
//
//     // 这种情况下会提供一条更有用的错误信息
//     let tt = multiply_v1("t", "2");
//     print(tt);
// }
//
// // 幸运的是，Option的map、and_then、以及很多其他组合算子也为Result实现了
// // 下面除了写法外，与上面那个完全一致
// // 它的作用是：如果值是合法的，计算其乘积，否则返回错误
// fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str
//             .parse::<i32>()
//             .map(|second_number| first_number * second_number)
//     })
// }
//
// fn print_v2(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n) => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
//
// pub fn for_result_map_v2() {
//     let twenty = multiply_v2("10", "2");
//     print_v2(twenty);
//
//     // 这种情况下就会提供一条更有效的信息
//     let tt = multiply_v2("t", "2");
//     print_v2(tt);
// }
//
// // 为带有错误类型ParseIntError的Result定义一个泛型别名
// type AliasedResult<T> = Result<T, ParseIntError>;
//
// fn multiply_v3(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str
//             .parse::<i32>()
//             .map(|second_number| first_number * second_number)
//     })
// }
//
// pub fn for_result_alias() {
//     print_v2(multiply_v3("10", "2"));
//     print_v2(multiply_v3("t", "2"));
// }
//
// // 如果发生了错误我们可以停止函数的执行然后返回错误，对有些人来说，这样的代码更好写，更易读
// fn multiply_v4(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     let first_number = match first_number_str.parse::<i32>() {
//         Ok(first_number) => first_number,
//         Err(e) => return Err(e),
//     };
//
//     let second_number = match second_number_str.parse::<i32>() {
//         Ok(second_number) => second_number,
//         Err(e) => return Err(e),
//     };
//
//     Ok(first_number * second_number)
// }
//
// pub fn for_result_v4() {
//     print_v2(multiply_v4("10", "2"));
//     print_v2(multiply_v4("t", "2"));
// }
//
// // ?问号几乎就等于一个会返回Err而不是panic的unwrap
// fn multiply_v5(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     let first_number = first_number_str.parse::<i32>()?;
//     let second_number = second_number_str.parse::<i32>()?;
//     Ok(first_number * second_number)
// }
//
// pub fn for_result_v5() {
//     print_v2(multiply_v5("10", "2"));
//     print_v2(multiply_v5("t", "2"));
// }
//
// // try!已经过时
// // fn multiply_v6(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError>{
// //     let first_number = try!(first_number_str.parse::<i32>());
// //     let second_number = try!(first_number_str.parse::<i32>());
// //     Ok(first_number * second_number)
// // }
//
// fn double_first(vec: Vec<&str>) -> i32 {
//     let first = vec.first().unwrap(); // 生成错误1
//     2 * first.parse::<i32>().unwrap() // 生成错误2
// }
//
// pub fn for_m_error_types() {
//     let number = vec!["42", "93", "18"];
//     // let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];
//
//     println!("The first doubled is {}", double_first(number));
//     // println!("The first doubled is {}", double_first(empty));
//     // println!("The first doubled is {}", double_first(strings));
// }
//
// // 处理混合错误类型的最基本的手段就是让他们互相包含
// fn double_first_v1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
// }
//
// pub fn for_option() {
//     let number = vec!["42", "93", "18"];
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];
//
//     println!("The first doubled is {:?}", double_first_v1(number));
//     println!("The first doubled is {:?}", double_first_v1(empty));
//     println!("The first doubled is {:?}", double_first_v1(strings));
// }
//
// // 有时候我们不想再处理错误，比如使用?的时候，但是如果Option是None则继续处理错误，一些组合算子可以让我们轻松的交换Result和Option。
// fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
//     let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
//     opt.map_or(Ok(None), |r| r.map(Some))
// }
//
// pub fn for_option_v2() {
//     let numbers = vec!["42", "93", "18"];
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];
//
//     println!("The first doubled is {:?}", double_first(numbers));
//     println!("The first doubled is {:?}", double_first(empty));
//     println!("The first doubled is {:?}", double_first(strings));
// }

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn for_define() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

// 如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们装箱Box，这样做的坏处就是，被包装的错误类型只能在运行时了解，而不能被静态的判断
// 取别名
type Result_Err<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn double_first_v2(vec: Vec<&str>) -> Result_Err<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // 装箱，下面的into也是装箱
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
}

fn print_v2(result: Result_Err<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn for_boxing() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

type Result_v1<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
struct EmptyVec_v1;

impl fmt::Display for EmptyVec_v1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec_v1 {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

// 这次没有把所有的Result和Options串起来
// 使用?问号立即得到内部值
fn double_first_v1(vec: Vec<&str>) -> Result_v1<i32> {
    let first = vec.first().ok_or(EmptyVec_v1)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_v1(result: Result_Err<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn for_reenter() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

// 包裹错误
type Result_v2<T> = std::result::Result<T, DoubleError_v2>;

#[derive(Debug)]
enum DoubleError_v2 {
    EmptyVec,
    //将错误包裹到自己的错误类型中
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError_v2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DoubleError_v2::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError_v2::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError_v2 {
    fn description(&self) -> &str {
        match *self {
            DoubleError_v2::EmptyVec => "empty vectors not allowed",
            DoubleError_v2::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DoubleError_v2::EmptyVec => None,
            DoubleError_v2::Parse(ref e) => Some(e),
        }
    }
}

// 实现从ParseIntError到DoubleError的转换
impl From<ParseIntError> for DoubleError_v2 {
    fn from(error: ParseIntError) -> Self {
        DoubleError_v2::Parse(error)
    }
}

fn double_first_v3(vec: Vec<&str>) -> Result_v2<i32> {
    let first = vec.first().ok_or(DoubleError_v2::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_v3(result: Result_v2<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error:{}", e),
    }
}

pub fn for_wrap_error() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

// 遍历Result
pub fn for_iter_result() {
    let strings = vec!["tofu", "93", "18"];
    // let possible_numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    // 使用filter_map会调用一个函数，过滤掉为None的所有结果
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .filter_map(result::Result::ok)
    //     .collect();

    // 使用collect使整个操作失败
    // Result实现了FromIter，因此，结果的向量Vec<Result<T, E>>可以被转换成结果包裹着向量Result<Vec<_>, E>
    // 同样的技巧可以对Option使用
    // let numbers: result::Result<Vec<_>, _> =
    //     strings.into_iter().map(|s| s.parse::<i32>()).collect();

    // 使用Partition()收集所有合法的值与错误
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(result::Result::is_ok);
    // 当你看着这些结果时，你会发现所有东西还在Result中保存着，要取出它们，还需要一些模版化的代码
    let numbers: Vec<_> = numbers.into_iter().map(result::Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(result::Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

//Rust强制实行资源获取即初始化，所以任何对象在离开作用域时，它的析构函数就被调用，然后它占有的资源就被释放
fn create_box() {
    //    在堆上分配一个整型数据
    let _box1 = Box::new(3i32);
    //    在这里_box1被销毁，资源得到释放，，在离开作用域时，这里就将要离开作用域了
}

fn t() {
    //    在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    //    嵌套作用域

    {
        let _box3 = Box::new(4i32);
    }

    //    创建一大堆Box，完全不需要手动释放内存
    for _ in 0..1_000 {
        create_box();
    }
    //    _box2在这里被销毁，内存得到释放，，在离开作用域时，这里就将要离开作用域了
}

// //析构函数
// struct ToDrop;
//
// impl Drop for ToDrop {
//     fn drop(&mut self) {
//         println!("ToDrop is being dropped");
//     }
// }
//
// fn main() {
//     let x = ToDrop;
//     println!("Made a ToDrop!");
//     //    当main函数的变量离开作用域，自定义的析构函数就会被调用
// }

//因为变量要负责释放它们拥有的资源，所以资源只能拥有一个所有者，这也防止了资源的重复释放，注意，并非所有变量都拥有资源
//在进行赋值或者通过值来传递函数参数的时候，资源的所有权会发生转移，在Rust里，这杯成为Move，移动
//在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针的产生

//此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    //    c被销毁并且内存得到释放
}

pub fn for_move() {
    //栈分配的整型
    let x = 5u32;
    //将x复制到y——不存在资源的移动
    let y = x;

    println!("x is {} and y is {}", x, y);

    //    a是一个指向堆分配的整数的指针
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // 移动a到b
    // 把a的指针地址复制到b，现在两者都指向同一个堆分配的数据，但是现在是b拥有它
    let b = a;
    // a不能访问数据
    // println!("a contains: {}", a);

    //此函数从b中取得堆分配的内存的所有权
    destroy_box(b);
    // println!("b is {}", b);
}

//可变性，当所有权转移时，数据的可变性可能发生改变
pub fn for_mut() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    //可变性错误
    // *immutable_box = 4;

    //移动box，改变所有权(和可变性)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}

//多数情况下，我们更希望能访问数据，同时不取得其所有权，为了实现这点，Rust使用了借用borrowing机制
//对象可以通过引用&T来传递，从而取代通过值T来传递

//此函数取得一个box的所有权并性销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destorying box that contains {}", boxed_i32);
}

//此函数借用了一个i32类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

pub fn for_borrow() {
    //    创建一个装箱的i32类型，以及一个存在栈中的i32类型
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

//此函数接受一个对Book类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

//此函数接受一个对可变的Book类型的引用，它将年份改为2014年
fn new_edition(book: &mut Book) {
    book.year = 2020;
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

pub fn for_mut1() {
    let immutabook = Book {
        author: "Douagsadflkj",
        title: "dsafagasgsd",
        year: 1967,
    };

    //    创建一个immutabook的可变拷贝
    let mut mutabook = immutabook;

    //    不可变地借用一个不可变对象
    borrow_book(&immutabook);
    //    不可变地借用一个可变对象
    borrow_book(&mutabook);

    //    可变地借用一个可变对象
    new_edition(&mut mutabook);
    //    可变地借用给一个不可变对象——错误
    // new_edition(&immutabook);
}

//当数据被不可变地借用时，它还会冻结freeze，已冻结的数据无法通过原始对象来修改，直到对这些数据的所有引用离开作用域为止
pub fn for_freeze() {
    let mut _mutable_integer = 7i32;

    {
        let large_integer = &_mutable_integer;
        // _mutable_integer = 50; // 报错，在被作用域被冻结
        println!("Immutably borrowed {}", large_integer);
    }

    //    正常运行，在这个作用域没有冻结
    _mutable_integer = 3i32;
}

//数据可以进行多次不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用，另一方面，在同一时刻内只允许有一个可变借用。
//只有在可变引用离开作用域之后，原始数据才可再次被借用
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn for_alias() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;
        //    通过引用和原始所有者来访问数据
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y, borrowed_point.z
        );

        // 不能可变的借用point，因为它现在有不可变的借用，出作用域才可以可变的借用，因为数据在不可变的借用时，已经被冻结了
        // let mutable_borrow = &mut point;
        // 此处再次使用被借用的值
        println!(
            "Point has corrdinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );
        // 到这里，上面的不可变借用才走出作用域，这行之后，数据才被解冻，可以声明和使用不可变借用
    }

    {
        let mut mutable_borrow = &mut point;

        //    通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错，不能打印，不能不可变的借用point，因为现在它有可变借用了
        // let y = &point.y;

        // 报错，不能打印，因为println会创建一个不可变引用
        // println!("Point Z coordinate is {}", point.z);

        // 可以工作，可变引用可以作为不可变的传给println
        println!(
            "Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );

        //    可变引用离开作用域
    }
    // 现在又可以不可变地借用point了
    let borrowed_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}

//通过let绑定来进行模式匹配或者解构时，ref关键字可以用来创建结构体/元组的字段的引用。
#[derive(Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

pub fn for_ref() {
    let c = 'Q';

    // 赋值语句左边的ref关键字等价于右边的 & 符号
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point2 = Point2 { x: 0, y: 0 };

    // 在解构一个结构体时，ref同样有效
    let _copy_of_x = {
        //解构结构体
        let Point2 {
            x: ref ref_to_x,
            y: _,
        } = point2;
        // 返回一个point的x字段的拷贝
        *ref_to_x
    };

    let mut mutable_point2 = point2;

    {
        let Point2 {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point2;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point2.x, point2.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point2.x, mutable_point2.y
    );

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构mutable_tuple来改变last的值
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

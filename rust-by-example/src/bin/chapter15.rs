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

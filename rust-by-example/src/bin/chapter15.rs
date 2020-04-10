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

//析构函数
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
    //    当main函数的变量离开作用域，自定义的析构函数就会被调用
}

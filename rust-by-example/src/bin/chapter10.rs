mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function()");
    }

    pub fn function() {
        println!("called my_mod::function()")
    }

    pub fn indirect_access() {
        print!("called indirect_access");
        private_function();
    }

    //模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        pub(in crate::bin::chapter10::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::public_function_in_my_mod");
            public_function_in_nested();
        }

        //只在当前模块中可见
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        //只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    //只在当前crate中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    //    嵌套模块的可见行遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called function()");
}

pub fn for_visibility() {
    //    模块机制消除了相同名字的项之间的歧义
    function();
    my_mod::function();

    //    公有项，包括嵌套模块内的，都可以在父模块外部访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}

//结构体的可见行
mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        // pub contents: T,
        contents: T,
    }

    impl<T> ClosedBox<T> {
        //    一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

pub fn for_pub_stru() {
    let open_box = my::OpenBox {
        contents: "public information",
    };
    // 字段可以被正常访问到
    println!("The open box contaions: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造
    // let closed_box = my::ClosedBox {
    //     contents: "classified information",
    // };

    // 不过带有私有字段的结构体可以使用公有的构造器来创建
    let _closed_box = my::ClosedBox::new("classified information");
    // 一个结构体中的私有字段不能被访问到
    // println!("The closed box contaions:{}", _closed_box.contents);
}

//use声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问
use crate::bin::chapter10::deeply::nested::function2 as other_function;

fn function2() {
    println!("called function2()");
}

mod deeply {
    pub mod nested {
        pub fn function2() {
            println!("called deeply::nested::function2()");
        }
    }
}

pub fn for_use() {
    // 更容易访问function
    other_function();
    println!("Entering block");
    {
        // 这和use deeply::nested::function2 as function2等价
        use deeply::nested::function2;
        // 此function2将掩蔽外部的同名函数
        function2();
        // use绑定具有局部作用域，在这个例子中，function2的掩蔽只存在于这个代码块中
        println!("Leaving block");
    }
    function2();
}

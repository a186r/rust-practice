macro_rules! say_hello {
    () => {
        // 此宏将展开成这个代码块里的内容
        println!("hello");
    };
}

pub fn for_macros() {
    say_hello!();
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}", stringify!(&func_name))
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} == {:?}", stringify!($expression), $expression)
    };
}
pub fn for_designators() {
    foo();
    bar();
    print_result!(1u32 + 1);

    // 代码块也是表达式
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left & $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

pub fn for_overload() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

macro_rules! find_min {
    // 基本情形
    ($x:expr) => {
        $x
    };
    ($x: expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}

pub fn for_repeat() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

// 不写重复代码
macro_rules! assert_equal_len {
    ($a:ident, $b:ident, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}:dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        )
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

// op!(add_assign, Add, +=, add);
// op!(mul_assign, Mul, *=, mul);
// op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    // 测试
    test!(add_assign, 1u32, 2u32, 3u32);
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            // 强制类型为整型
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

pub fn for_dsl() {
    calculate! {
        eval 1+2
    }

    calculate! {
        eval (1+2) * (3/4)
    }
}

// 可变参数接口，可变参数可以接收任意数目的参数
macro_rules! calculate2 {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // 递归的拆解多重的eval
    (eval $e:expr, $(eval $es:expr), +) => {
        {
            calculate2!{eval $e}
            calculate2!{$(eval $es), +}
        }
    };
}

pub fn for_variadics() {
    calculate2! {
        eval 1+2,
        eval 3+4,
        eval (2*3)+ 1
    }
}

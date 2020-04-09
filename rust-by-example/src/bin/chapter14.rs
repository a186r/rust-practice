// struct A;
//
// struct Single(A);
//
// struct SingleGen<T>(T);
//
// pub fn for_generics() {
//     let _s = Single(A);
//
//     let _char: SingleGen<char> = SingleGen('a');
//
//     //SingleGen的类型参数也可以隐式的指定
//     let _t = SingleGen(A);
//     let _i32 = SingleGen(5);
//     let _char = SingleGen('b');
// }
struct A; // 具体类型A
struct S(A); // 具体类型S
struct SGen<T>(T); // 范型类型SGen

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}
//这个函数是关于T的范型函数
fn generic<T>(_s: SGen<T>) {}

pub fn for_gen() {
    //    使用非范型函数
    reg_fn(S(A));
    //隐式的指定类型参数A
    gen_spec_t(SGen(A));
    //隐式的指定类型参数i32
    gen_spec_i32(SGen(12));

    //为generic显式的指定类型参数char
    generic::<char>(SGen('a'));
    generic(SGen('b'));
}

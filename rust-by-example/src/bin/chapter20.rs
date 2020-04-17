use core::num::dec2flt::rawfp::encode_normal;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{env, fs, io, thread};

static NTHREADS: i32 = 10;

// 这是主线程
fn main() {
    // let mut children = vec![];
    //
    // for i in 0..NTHREADS {
    //     // 启动另一个线程
    //     children.push(thread::spawn(move || {
    //         println!("this is thread number {}", i)
    //     }));
    // }
    //
    // for child in children {
    //     // 等待线程结束，返回一个结果
    //     let _ = child.join();
    // }
    let data = "86967897737416471853297327050364959
11861322 5 7 5 564 7 2 396 3 29 754262 4 9 6 28 5 0 
70856234701860851907960690014725639
38397966707106094172783238747669219
523807952578882365254 5 9 3 0 3 33 0 3 02837
584953271357           44041048  897885734297812
69920216438980 873548 808413720 956532
16278424637452  5 8 9 8 6 0  34537482857 4668";

    let mut children = vec![];

    // 数据分段，每段都是完整数据的一个引用
    let chunked_data = data.split_whitespace();
    // enumerate会把当前的迭代计数与被迭代的元素以元组的形式返回
    for (i, data_segment) in chunked_data.enumerate() {
        for i in 0..NTHREADS {
            // 启动线程
            children.push(thread::spawn(move || -> u32 {
                // 计算该段的每一位的和
                let result = data_segment
                    // 对该字段中的字符进行迭代
                    .chars()
                    // 把字符转成数字
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    // 对返回的数字类型的迭代器求和
                    .sum();
                // println会锁住标准输出，这样个线程打印的内容不会交错在一起
                println!("processed segment {}, result={}", i, result);
                result
            }));
        }
    }

    /**
     * Reduce阶段
     * 收集中间结果，得出最终结果
     **/
    let mut intermediate_sums = vec![];

    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // 把所有中间结果加起来，得到最终结果
    // 涡轮鱼写法——推荐
    let final_result = intermediate_sums.iter().sum::<u32>();
    // 不使用涡轮鱼写法，显式的指定类型
    // let final_result: u32 = intermediate_sums.iter().sum();
    println!("Final sum result: {}", final_result);
}

// Rust为线程之间的通信提供了异步的通道，通道允许两个端点之间信息的单向流动
static NTHREADS_V2: i32 = 3;

pub fn for_channels() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS_V2 {
        // 通道可以被复制
        let thread_tx = tx.clone();

        // 每个线程都通过通道来发送它的id
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    let mut ids = Vec::with_capacity(NTHREADS_V2 as usize);
    for _ in 0..NTHREADS_V2 {
        // recv从通道中拿到一个消息
        // 若无消息可用的话，recv将阻止当前线程
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}

pub fn for_path() {
    let path = Path::new(".");
    let display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
    println!("{:?}", display);
}

pub fn for_open() {
    let path = Path::new("hello.txt");
    let display = path.display();

    // 以只读的方式打开路径
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn's read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

// 创建文件
static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

// pub fn for_create() {
//     let path = Path::new("out/lorem_ipsum.txt");
//     let display = path.display();
//
//     // 以只写模式打开文件，返回io::Result<File>
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create{]: {}", display, why),
//         Ok(file) => file,
//     };
//
//     match file.write_all(LOREM_IPSUM.as_bytes()) {
//         Err(why) => panic!("sdfasdfasfsa {}: {}", display, why),
//         Ok(_) => println!("{}", display),
//     }
// }
//
// // 读取行
// pub fn for_read_lines() {
//     if let Ok(lines) = read_lines("./hosts") {
//         for line in lines {
//             if let Ok(ip) = line {
//                 println!("{}", ip);
//             }
//         }
//     }
// }

// 输出包裹在Result中以允许匹配错误
// 将迭代器返回给文件行的读取器Reader
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// 子进程
pub fn for_process() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}

// 管道
static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
pub fn for_pipe() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // 将字符串写入wc的stdin
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => println!("wc responded with:\n{}", s),
    }
}

// 等待
pub fn for_wait() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}

// 文件系统操作
// '% cat path'的简单实现
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn for_fs() {
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| println!());

    // 递归的创建一个目录
    fs::create_dir_all("a/b/c").unwrap_or_else(|why| println!());

    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| println!());

    // 读取目录的内容
    match fs::read_dir("a") {
        Err(why) => println!("{}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    // 删除一个文件
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| println!());

    // 移除一个空目录
    fs::remove_dir("a/c/d").unwrap_or_else(|why| println!("{}", why.kind()));
}

// 程序参数
pub fn for_arg() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

// 可以使用模式匹配来解析简单的参数
fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
    );
}

pub fn for_matching() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!(""),
        2 => match args[1].parse() {
            Ok(42) => println!("answer"),
            _ => println!("noe answer"),
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // 解析数字
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!();
                    help();
                    return;
                }
            };

            // 解析命令
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("err");
                    help()
                }
            }
        }

        // 所有其他情况，显示帮助信息
        _ => help(),
    }
}

// 外部语言函数接口
extern "C" {
    // 用于计算单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;
    //用来计算单精度复数的复变余弦
    fn ccosf(z: Complex) -> Complex;
}

// 调用其他语言的函数被认为是不安全的，所以我们通常给它们写一层安全的封装
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

pub fn for_ffi_() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 调用外部语言函数是不安全操作
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // 调用不安全操作的安全的 API 封装
    println!("cos({:?}) = {:?}", z, cos(z));
}

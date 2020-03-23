use std::{io, thread};
use lazy_static::lazy_static;
use std::sync::RwLock;

mod file;

//在此申明变量
//lazy_static可以帮助实现延迟初始化static常量的功能。
//rust静态项是一种全局变量，类似于常量，到那时静态项不内联使用，每个在值只对应一个实例，并且在内存中只有一个固定的地址。
//任何存储在static的类型都必须是Sync
//很多情况下我们希望延迟初始化静态量，只有在第一次访问的时候或者在某个特定的时候才初始化它，那么就可以使用lazy_static
lazy_static! {
    static ref FILE1: RwLock<String> = RwLock::new(String::from(""));
    static ref FILE2: RwLock<String> = RwLock::new(String::from(""));
}

fn main() -> io::Result<()> {
    println!("program started");

    let thread_1 = thread::spawn(|| {
        let mut w1 = FILE1.write().unwrap();
        *w1 = file::read_file("src/file1.txt").unwrap();
        println!("read file 1");
    });

    println!("Launched Thread 1");

    let thread_2 = thread::spawn(|| {
        let mut w2 = FILE2.write().unwrap();
        *w2 = file::read_file("src/file2.txt").unwrap();
        println!("read file 2");
    });

    println!("Launched Thread 2");

    let mut rf1: bool = false;
    let mut rf2: bool = false;

    // loop {
    //     // read()
    //     let r1 = FILE1.read().unwrap();
    //     let r2 = FILE2.read().unwrap();

    //     if *r1 != String::from("") && rf1 == false{
    //         println!("completed file 1");
    //         rf1 = true;
    //     }

    //     if *r2 != String::from("") && rf2 == false{
    //         println!("completed file 2");
    //         rf2 = true;
    //     }
    // }

    // 现在执行方式有所不同，如果file1.txt比file2.txt大得多，则应该先处理第二个文件。
    loop{
        // read
        let r1 = FILE1.try_read();
        let r2 = FILE2.try_read();

        match r1 {
            Ok(v) => {
                if *v != String::from("") && rf1 == false {
                    println!("completed file 1");
                    rf1 = true;
                }
            },

            Err(_) => {},
        }

        match r2 {
            Ok(v) => {
                if *v != String::from("") && rf2 == false {
                    println!("completed file 2");
                    rf2 = true;
                }
            },

            Err(_) => {},
        }
    }

    Ok(())
}
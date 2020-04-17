use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

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

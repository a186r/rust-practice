fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

pub fn for_dead_code() {
    used_function();
}

// 这个函数仅当目标系统是Linux的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统不是Linux时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

pub fn for_cfg() {
    are_you_on_linux();

    println!("Are you  sure?");

    if cfg!(target_os = "linux") {
        println!("Yes, It's definitely linux");
    } else {
        println!("Yes, It's definitely not linux");
    }
}

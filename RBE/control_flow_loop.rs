/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 11:01:08
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-19 11:46:39
 * @FilePath: /RustConqueror/RBE/control_flow_loop.rs
 * @Description: Rust的loop(嵌套)循环
 * 
 * Rust provides a loop keyword to indicate an infinite loop.
 * 
 * The break statement can be used to exit a loop at anytime, whereas the continue statement can be
 * used to skip the rest of the iteration and start a new one.
 * 
 * In Rust，关键字loop意味着无限循环(等价于 while true)，不过 break 和 contine 和其它编程语言的功能是一样。（Happy~）
 * 
 * loop表达式(loop expression)。
 * 
 * 根据下面的示例，我们可以看出Rust在循环方面还是玩出了新花样。
 * 1. 关键字loop；
 * 2. 嵌套循环使用标签+loop。'label_name:loop{}
 * 3. 从循环中返回值. break 返回值;
 * 
 */
fn main(){
    normal_loop();

    nested_loop();

    return_loop();
}

/**
 * 注意编程语言中loop和iteration的区别
 */
fn normal_loop(){
    let mut count = 0usize;
    println!("Let's count until infinity！");

    loop {
        count += 1;
        
        if count == 5 {
            println!("It's Five");
            // Skip the rest o this iteration(跳出本次循环)
            continue;
        }

        println!("It's {}", count);

        if count ==  10 {
            println!("OK,that's enough!", );
            // Exit this loop(退出循环)
            break;
        }
    }
}

/***
 * 嵌套循环(nested loop),必须用标签(label)声明,语法也很奇特('label_name).
 * 显然，break & continue 也是需要指定对应的标签才能处理对应的循环。
 * 
 * (对于有经验的开发者而言，这样的代码显然不是很友好，不利于后期维护，个人拙见)
 */
fn nested_loop(){
    'outer:loop {
        println!("Welcome to the outer loop");

        'inner:loop {
            println!("Welcome to the inner loop");
            // 跳出内部循环(This would break only the inner loop)
            // break;

            break 'outer;// 跳出外循环
        }
        println!("This point will never be reached");//unreachable statement
    }

    println!("Exited the outer loop");
}


/**
 * One of the uses of a loop is to retry an operation until it succeeds.
 * (循环的用途之一是重试操作，直到操作成功为止。)
 * 
 * 满足一定条件时，退出循环并返回值。
 * In Rust,我们可以通过将返回值放在break之后来达到此目的。
 */
fn return_loop(){
    let mut score = 0;

    let result = loop {
        score += 20;
        if score == 100 {
            break "Full Mark!!!"; 
        }
    };

    println!("Bertking get {}",result);
}
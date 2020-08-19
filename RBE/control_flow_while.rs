/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 14:10:17
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-19 14:19:59
 * @FilePath: /RustConqueror/RBE/control_flow_while.rs
 * @Description: while循环
 * 
 * In Rust, while 循环跟其它语言没有太大的差别。值得我们注意的是：Rust的布尔条件都不能用括号包裹。
 * 这里结合着if-else,来解决一道Fizz Buzz问题(100以内)
 */
fn main(){
    while_fizz_buzz();
}

fn while_fizz_buzz(){
    let mut i = 1;

    while i <= 100 {
        if i % 15 == 0 {
            println!("FizzBuzz!");
        }else if i % 3 == 0 {
            println!("Fizz!");
        }else if i % 5 == 0 {
            println!("Buzz!")
        }else {
            println!("i = {}",i);
        }

        i += 1;
    }
}



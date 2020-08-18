/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-18 20:13:08
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-18 20:51:09
 * @FilePath: /RustConqueror/TRPL/control_flow_loop.rs
 * @Description: Rust中的循环控制
 * 1. loop
 * 2. while
 * 3. for
 */
fn main(){
    loop_function();

    while_function();

    iter_collection_while();

    iter_collection_for();

    iter_range_for();
}

fn loop_function(){
    let mut i = 0;
    loop{
        if i == 3 {
            break
        }else {
            println!("{} Go!",i );
        }
        i += 1;
    }
}

fn while_function(){
    let mut i = 3;
    while i>0 {
        println!("Smaller");
        i -= 1;
    }
    println!("Game Over...");
}
 /**
 *  使用while遍历集合在任何一种编程语言中，都不能算是高明的做法。(不推荐)
 */
fn iter_collection_while(){
    let array = [1,2,3,4,5];
    let mut index = 0;
   
    while index < array.len() {
        println!("The index {} value is:{}",index,array[index]);
        index += 1;
    }
}
/**
 * The safety and conciseness of for loops make them the most commonly used loop construct in Rust. 
 * 正因为for循环的安全和简洁性使for成为Rust最流行的循环方式。(for在很多语言中都很流行啊~~2333~~)
 * 
 * Rust在用法上跟Kotlin差不多啊，哈哈~
 */
fn iter_collection_for(){
   let array = ["First","Second","Third","Fourth","Fifth","Sixth"];
   for element in array.iter() {
       println!("The value is {}",element);
   }
}

/**
 * for在Range中的使用也很普遍
 * 需要注意的是Rust中的(a..b)不包括b。这和Kotlin居然不一样。~~呜呜
 */
fn iter_range_for(){
    for i in (1..5) {
        println!("{}",i)
    }
}

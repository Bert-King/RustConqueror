/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-27 15:07:36
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-27 15:22:57
 * @FilePath: /RustConqueror/RBE/functions.rs
 * @Description: Rust中的函数部分
 * 
 * Rust中以关键字fn来声明函数。使用箭头符号指定返回值: ->
 * 
 * 返回值：The final expression in the function will be used as a return value. Alternatively,the return statement
 * can be used to return a value earlier from within the function, even from inside loops or if s.
 * 
 * 简而言之：函数的最后一个表达式可以作为函数的返回值。
 * 另外也可以使用return语句。
 * 
 * 如果函数没有返回值，其实返回的是: unit type () 默认被省略掉了。
 * 
 */


 /**
  * 常规的无返回值的函数
  */
fn nothing(){

}


/**
 * 作为参考，一般不这么写
 */
fn nothing2() -> () {

}


/**
 * 表达式不能加 分号;
 */
fn sum(a:isize,b:isize) -> isize {
    a + b // 表达式,
}

fn sum2(a:isize,b:isize) -> isize {
    return a+b;
}




fn main(){
    println!("{:?}",nothing()); // 结果为: ()
    println!("{:?}",nothing2()); // 结果为: ()

    println!("{}",sum(1,2)); // 3
    println!("{}",sum2(1,2)); // 3

}



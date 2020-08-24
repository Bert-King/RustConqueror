/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-24 16:22:44
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-24 17:52:11
 * @FilePath: /RustConqueror/TRPL/error_handling.rs
 * @Description: Rust的错误处理
 * In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
 * (对于大多数场景,Rust在代码编译时，尽可能地帮我们发现错误)
 * 
 * Rust将错误分为两类:
 * 1. recoverable ---》 Result<T,E>;
 * 2. unrecoverable ---》 panic! 宏(停止运行); index out of bounds
 */


 use std::fs::File;
 use std::io::ErrorKind;
fn main(){
   // panic!("crash and burn");

   //let v = vec![1,2,3];
  // v[99];

  error_handling_unwrap();

}


fn error_handling_match(){
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file :{:?}",e),
            },
  
            other_error => {
                panic!("Problem opening the file : {:?}",other_error)
            }
        },
        
    };
  
}

fn error_handling_unwrap(){
    let f = File::open("HelloWorld.txt").unwrap();
}
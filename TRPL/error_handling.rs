/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-24 16:22:44
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-24 20:55:56
 * @FilePath: /RustConqueror/TRPL/error_handling.rs
 * @Description: Rust的错误处理
 * In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
 * (对于大多数场景,Rust在代码编译时，尽可能地帮我们发现错误)
 * 
 * Rust将错误分为两类:
 * 1. recoverable ---》 Result<T,E>;
 * 2. unrecoverable ---》 panic! 宏(停止运行); index out of bounds
 * 
 * 我们在写代码的时候如何决定选择返回Result? 还是 panic! ?
 * 
 * 通常情况下，返回Result将会是一个好的选择。极少情况下，才会用panic!.
 * 
 * 
 * 
 *  What you need to know right now is that T represents the type of the value that will be returned in a success case within the Ok variant, 
 *  and E represents the type of the error that will be returned in a failure case within the Err variant. 
 * 
 */


 use std::fs::File;
 use std::io::{self,ErrorKind,Read};
 use std::env;
fn main(){
   // panic!("crash and burn");

   //let v = vec![1,2,3];
  // v[99];

  let args:Vec<String> = env::args().collect();
  println!("{:?}",args);

//   error_handling_unwrap();

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

/**
 * If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
 * If the Result is the Err variant, unwrap will call the panic! macro for us.
 * 
 * 对于unwrap()，如果结果OK，则返回结果，否则返回Panic!.
 */
fn error_handling_unwrap(){
    let f = File::open("HelloWorld.txt").unwrap();
}
/**
 * expect, which is similar to unwrap, lets us also choose the panic! error message. 
 * expect()方法在Panic时，允许我们自定义错误信息
 */
fn error_handling_expect(){
    let f = File::open("hello.txt").expect("Failed to open HelloWorld.txt");
}


/**
 * Rust的错误也是可以往外层抛的。
 * Rust为了便于我们处理这种情况，特地为我们提供了运算符(?)
 */
fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 注意这里
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/**
 * 操作符？来传播错误
 */
fn read_username_from_file2() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file3() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


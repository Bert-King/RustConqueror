/*
 * 因为Rust是基于表达式的语言.我们有必要再梳理一下Statements(声明or语句)和Expressions(表达式)的区别
 *  Statements are instructions that perform some action and do not return a value. 
 *  (语句是运行某些行为的指令，没有返回值)
 *   
 *   Expressions evaluate to a resulting value. 
 *   (表达式的计算结果就是其结果值(可理解为：表达式的计算结果就是该表达式的返回值))
 * 
 * 理解两者的区别和联系将有助于我们更好地理解Rust的函数
 */
fn main() {
   
    statements();

    expressions();

}

/**
 * 语句
 */ 
fn statements(){
    let number = 6; // variable declaration using `let` is a statement(使用let进行变量声明 是一个语句)
    // let value = (let number = 6); error: expected expression, found statement (`let`)
}
/*
 * 表达式 
 * >1. 表达式可以是语句的一部分(Expressions can be part of statements)。
 * >2. 表达式不包括结尾的分号.如果在一个表达式加上分号将会变成一个语句。
 * >3. 函数调用是表达式(Calling a function is an expression)。
 * >4. 宏调用是表达式(Calling a macro is an expression)。
 * >5. 我们用{}生成的块也是表达式(The block that we use to create new scopes,{},is an expression)。
 */
fn expressions(){
 let count = 5; // 此处的5就是表达式

 let sum = 5 + 6; // 此处的5+6也是表达式

 let num = {
     let index = 0;
     index + 1 // 注意这里没有分号;(semicolon)
 };

 println!("num = {}", num );
}
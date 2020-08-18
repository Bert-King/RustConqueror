
/*
 * 由于Rust的常量必须指定变量类型，所以Rust的教程中最好先讲数据类型再讲变量应该更好一些吧。(个人拙见！！！)
 */
fn main(){
    immutable_variable();
    mutable_variable();
    constants();
}

/*
 * In Rust, variables are immutable only by default.
 * (在Rust中，变量的值默认为不可变的)
 */
fn immutable_variable(){
    let x = 0;
    // x = 1; //cannot assign twice to immutable variable `x`
    println!("x = {}", x);
}

/*
 * Sometimes mutablility can be very useful.
 * However, we still have the option to make variables mutable.
 * 
 * We can make variables mutable by adding mut in front of the variable name.
 * (我们可以通过在变量名前面加上关键字mut，让变量的值可变(值可变而类型不可变))
 */
fn mutable_variable(){
    let mut x = 0;
    // x = "Bert"; // expected integer, found `&str`
    x = 100;
    println!("x = {}", x);
}

/*---我们了解到Rust的变量及其可变性的内容后，对于有经验的开发者而言，心中不免有些疑虑？我这里斗胆先揣测各位的心思---*/
/*
 * 1. 既然变量不可变，那Rust有常量的概念吗？变量和常量的有什么区别呢？
 * 
 * 首先Rust中有常量的概念。
 * 两者的区别：
 * > 1. 常量不能被关键字mut所修饰，这也意味着常量永远不会变。
 * > 2. 常量用关键字const修饰且必须指定类型。
 * > 3. 常量可以在任何范围内声明。(一般全局范围会很有用)
 * > 4. 常量只能被常量表达式赋值，而不能是函数返回结果or需要在运行时计算的表达式。
 * > 5. 常量一般用大写字母+下划线的格式命名。并且可以在数字中加入下划线来提高可读性。
 * 
 */

 fn constants(){
     const PI:f64 = 3.1415926;
     println!("PI = {}", PI);

     const MONEY_ACCOUNT:u64 = 1_000_000_000_000_000_000;
     println!("银行账户的金额 = {}",MONEY_ACCOUNT );
 }





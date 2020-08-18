fn main(){
    without_parameters();

    parameters("Bertking");

    println!("age = {}",get_age() );

    println!("area = {}", area(3.0) );

    println!("name = {}",get_name() );
}

/**
 * 无参且无返回值的函数
*/
fn without_parameters(){
    println!("Hello,Rust" );
}

/**
 * 有参数但无返回值的函数
*/
fn parameters(name:&str){
    println!("Hello,{}",name);
}

/*
* 无参有返回值的函数
*/
fn get_age() -> u64 {
    25 //必须是表达式，即如果不使用return语句就不能有分号
}

/*
 * 有参且有返回值
*/
fn area(radius:f64) -> f64 {
    return radius * radius * 3.1415926;
}
/*
 * Rust和Erlang一样，在处理字符串时，让人有点懵逼...
*/
fn get_name() -> String {
    return "Bertking".to_string();
}

/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 15:57:03
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 16:18:11
 * @FilePath: /RustConqueror/RBE/std_hashmap.rs
 * @Description: Rust中的HashMap
 * 
 * Where vectors store values by an integer index, HashMaps store values by key
 * (Vector根据整数下标存储值，而HashMap根据key存储值。)
 * HashMap keys can be booleans, integers, strings, or any other type that implements the Eq and Hash traits. 
 * (HashMap的key可以是booleans,integers,strings,或者任何实现Eq和Hash特性的其它类型)
 * 
 * 创建HashMap的方式:
 * 1. HashMap::with_capacity(uint)
 * 2. HashMap::new() //推荐
 * 
 * 注意Rust中使用key需要使用&。
 * insert()方法返回的是Option<T>
 * 
 * 
 */

use std::collections::HashMap;

 fn main(){

    let mut langs = HashMap::new();

    langs.insert("J", "Java");
    langs.insert("R", "Rust");
    langs.insert("C", "C++");
    langs.insert("E", "Erlang");
    langs.insert("P", "Python");
    langs.insert("G", "Go");

    println!("{:?}",langs);

    langs.remove(&"J");
    println!("{:?}",langs);

    for (shorthand,&lang) in langs.iter() {
        println!("Calling {}:{}",shorthand,lang);
    }

    println!("----------------------");
    match langs.get(&"G") {
        Some(&lang) => println!("Just find J:{}",lang),
        _ => println!("Don't have J"),
    }

     
 }

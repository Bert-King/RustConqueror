/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 20:50:19
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-20 10:29:58
 * @FilePath: /RustConqueror/TRPL/tuples.rs
 * @Description: Rust的元组(Tuple)
 * 
 * Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
 * 复合类型可以将多个值组合为一个类型。Rust有两种基本的符合类型：元组和数组。
 * (PS:如果这样的话，那结构体和枚举貌似也算是复合类型) 
 *
 * Erlang中的元组概念和这个一样，Python的记不太清了，貌似也差不多。
 * 
 * 元组的特点：
 * 1. 固定长度；
 * 2. 类型不固定；
 * 3. 使用parentheses();
 * 4. 分配在Stack上(仅限于元素类型都是基本类型的情况下)。
 * 5. 元组的访问：(两种方式：模式匹配和下标) 
 * 
 * 阅读并理解Ownership相关内容之后，将会进一步加深对Tuple的理解。
 */
fn main(){
    // 1. 声明元组
  let elements_tuple:(usize,f64,bool,&str) = (150,3.1415,true,"Bertking");
    // 2. 访问元组(模式匹配pattern match)
   let (weight,_pi,_gender,name) = elements_tuple;
   println!("体重 = {},名字 = {}",weight,name );

   // 3. 访问元组(使用下标)
   println!("圆周率 = {}，性别 = {}",elements_tuple.1, if elements_tuple.2 {"男"} else {"女"} );

}

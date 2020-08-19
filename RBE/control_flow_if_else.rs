/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 10:06:13
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-19 11:42:55
 * @FilePath: /RustConqueror/RBE/control_flow_if_else.rs
 * @Description: if_else语句的讲解
 * 
 * Branching with if-else is similar to other languages. 
 * Unlike many of them,the boolean condition doesn't need to be surrounded by parentheses,and each condition is followed by a block.
 * if-else conditionals are expressions,and, all branches must return the same type.
 * 私以为这几句话足以概括Rust的if-else表达式。(对有其它编程语言经验者而言)
 * 
 * 理解expressions and statements 对于理解Rust真的很有帮助哦。
 * 
 * 在if-else语句的讲解上，TRPL(The Rust Programming Language)要稍逊于RBE(Rust By Example)。
 * 
 */

 fn main(){
    neg_or_pos(-1);
    grade_level(100);
 }

 fn neg_or_pos(value:isize){
     if value > 0 {
        println!("{} is positive",value);
     } else if  value ==0 {
        println!("{} is zero",value);
     }else {
         println!("{} is negative",value);
     }
 }

 /**
  * if-else conditionals are expressions, and all branchs must return the same type.
  */
 fn grade_level(score:usize){
     let level = if score >= 95 && score <= 100{
         "A"
     }else if score >=85 && score < 95{
        "B"
     }else if score >=75 && score < 85 {
         "C"
     }else if score >=60 && score < 75 {
         "D"
     }else {
         "E"
     };
     println!("{} corresponding to {}",score,level);
 }
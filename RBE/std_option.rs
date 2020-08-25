/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 15:17:22
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 15:36:53
 * @FilePath: /RustConqueror/RBE/std_option.rs
 * @Description: Rust标准库中的Option(选项)
 * 有时候发生错误时，最好的方法是抓住程序的某些失败部分，而不是直接抛出panic!.这就是枚举Option的用武之地。
 * Sometimes it's desirable to catch the failure of some parts of a program instead of calling panic!; this can be accomplished using the Option enum.
 * 
 * Option<T> 枚举有两个变量：
 * 1. > None -> 预示着失败或者缺乏值(failure or lack of value)
 * 2. > Some(value) -> 包裹着T的value的元组结构
 * 
 */

 
 /**
  * 不会引入panic!的整数除法
  */
 fn checked_division(dividend:i32,divisor:i32) -> Option<i32>{
     if divisor == 0 {
         None
     }else {
         Some(dividend/divisor)
     }
 }

 fn try_division(dividend:i32,divisor:i32){
     match checked_division(dividend, divisor) {
         None => {
             println!("{}/{} failed!",dividend,divisor);
         },

         Some(quotient) => {
             println!("{}/ {} = {}",dividend,divisor,quotient);
         },
     }
 }


 fn main(){
     try_division(4, 2);
     try_division(3, 0);

     let optional_float = Some(0f32);
     println!("{:?} unwraps to {:?}",optional_float,optional_float.unwrap());

     let none:Option<i32> = None;
     println!("{:?} unwraps to {:?}",none,none.unwrap()); // panic!
 }
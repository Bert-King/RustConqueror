/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 21:05:14
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-20 11:27:34
 * @FilePath: /RustConqueror/TRPL/arrays.rs
 * @Description: Rust的数组
 * 数组方面的内容和其它编程语言基本相同。
 * 
 * Rust数组的特点:
 * 1. 元素类型必须相同；
 * 2. 固定长度;
 * 3. [](square bracket)
 * 4. 内存分配在栈上。(不需要刻意去记住，因为理解Ownership之后，自然就知晓其应该分配在什么地方(Stack or Heap))
 * 5. 编译期间就可以发现数组越界。
 * 
 * --------------------------------------------------------------------------------------------------
 * 
 * Rust数组的用途：
 * 1. 想让数据分配在Stack上。（Stack的速度比堆快）
 * 2. 想让(或者明确知道)数据的长度保持固定
 * 
 * -----------------------------------------
 * Rust数组的创建:
 * 1. []
 * 2. [T;size]
 * 
 * 访问数组元素：
 * 和其他编程语言一样，通过下标进行访问。(Rust在编译期间就可以发现数组越界)
 * 
 */

 fn main(){
     // 该种情况下属于：编译器可以自动推导出元素类型以及数组长度。(PS:自动推导在后来的语言上应该属于常规武器。)
     let months = ["January","February","March","April","May","June","July","August","September","October","November","December"];
     // 标准的数组类型声明[T;size],[]是其的一部分
     let days:[&str;7] = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];

     // 声明的size必须和[]中的元素个数对应。(明智的选择)
     // let numbers:[isize;5] = [1,2,3]; // expected an array with a fixed size of 5 elements, found one with 3 elements
     
    let zeros = [0;5];// 等价于 [0,0,0,0,0]

    println!(" 星期三 =  {}", days[2]);

    println!(" 星期八 ？ {}", days[7]);// 编译期间就可以发现数组越界，就问你强不强：index out of bounds:

 }

 
/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-26 15:45:47
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-26 16:26:56
 * @FilePath: /RustConqueror/RBE/attributes.rs
 * @Description: Rust中的属性Attributes
 * 
 * 叹号(bang) ---> !
 * 
 * 属性是可以被应用于一些Module，Crate，Items的元数据(metadata).其主要用途有:
 * An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
 * 1. conditinal compilation of code.（条件编译）
 * 2. set crate name, version and type(binary or library) (设置Crate名称，版本号，以及类型(二进制文件or库))
 * 3. disable lints(warnings)(禁用代码检查or警告)
 * 4. enable compiler features(macros,glob imports,etc.) （开启编译器的特性）
 * 5. link to a foreign library （链接外部库）
 * 6. mark functions as unit tests （标记函数为单元测试）
 * 7. mark functions that will be part of a benchmark. (标记函数为benchmark的一部分)
 * 
 * 语法：
 * 1. 作用于整个Crate ---> #![crate_attribute]
 * 2. 作用于Module或者Item ---> #[item_attribute]
 * 
 * 设置参数的方式:
 * #[attribute = "value"]
 * #[attribute(key = "value")]
 * #[attribute(value)]
 * 
 * 另外，Attributes可以含有多个值。
 * #[attribute(value,value2)]
 * #[attribute(value,value2,value3,
 *      value4,value5)]
 * 
 * ---------------------------------------------------------------
 * 实战环节:
 */

 fn used_function(){
     println!("This is a used function...");
 }

 /**
  * warning: function is never used: `unused_function`
  */
 #[allow(dead_code)]
 fn unused_function(){
     
 }

 fn main(){
     let v = 1i32;
     used_function();
 }
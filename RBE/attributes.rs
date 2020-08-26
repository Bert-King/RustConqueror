/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-26 15:45:47
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-26 17:52:09
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
 * #[allow(dead_code)] 用来禁止 dead_code(无效代码) lint。
 * Note that in real programs,you should eliminate dead code. 在实际项目中，我们应该尽可能地消除无效代码。
 * 
 * 
 * Crate中的属性：
 * 1. crate_type : 告知编译器该crate是二进制文件 OR 库文件(甚至是库的类型)
 * 2. crate_name : 设置crate的名称
 * 
 * However,it is important to note that both the crate_type and crate_name attributes have no effect whatsoever when using Cargo,the Rust package manager.
 * (但是需要我们注意的是，crate_type和crate_name属性在使用Cargo-Rust的包管理工具时没有任何效果的.由于Cargo被大量用于Rust的项目，这就意味着这两个属性在现实世界中很少能用上。) 
 * 
 * 另外需要注意的是，当使用crate_type属性时，使用rustc编译就不需要添加 --crate-type标志啦。
 * 
 */

 // 指定该Crate为库，且名称为attr.最后编译后的将会生成libattr.lib文件
 #![crate_type = "lib"]
 #![crate_name = "attr"]  



 fn used_function(){
     println!("This is a used function...");
 }

 /**
  * warning: function is never used
  * warning: unused variable
  * 上面的警告，大家都不陌生。这是编译器提供的dead_code lint.
  * 我们可以使用属性#[allow(dead_code)] 来禁止这个lint。
  * 
  */
 #[allow(dead_code)]
 fn unused_function(){
     
 }

 fn main(){
     let v = 1i32;
     used_function();
 }
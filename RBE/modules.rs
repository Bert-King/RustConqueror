/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-20 16:02:53
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-20 17:15:18
 * @FilePath: /RustConqueror/RBE/modules.rs
 * @Description: Rust的模块(感觉跟JS的差不多,跟Java相去甚远)
 * 
 * 模块这个概念，对于有其他经验的开发者绝对不会陌生。
 * 
 * Rust中的模块是:functions, structs, traits,impl blocks and other modules 的集合。
 * 在Rust中，一个项目按照范围的大小分为：
 * Packages ---> Crates ---> Modules(说实话看TRPL的这部分内容成功把我搞懵逼...)
 * 这里对于Crate补充一点：
 *  RBE上面的说明是完全符合Rust文档的。什么是Crate?即：将Rust源文件(.rs文件)经过编译器编译过的二进制文件。
 */

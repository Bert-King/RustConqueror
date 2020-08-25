/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 15:42:11
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 15:55:37
 * @FilePath: /RustConqueror/RBE/std_panic!.rs
 * @Description: Rust中的panic!宏
 * 
 * The simplest error handling mechanism we will see is panic. It prints an error message, starts unwinding the stack,
 * and usually exits the program.
 * (panic是Rust处理错误逻辑最简单的方式。)
 * 
 * 
 * The panic! macro can be used to generate a panic and start unwinding its stack.
 * (panic!宏被用来产生一个panic并开始展示其栈内容)
 * 
 * While unwinding, the runtime will take care of freeing all the resources owned by the thread by calling the destructor of all its object.
 * (展开时，运行时将通过调用其所有对象的析构函数来释放线程所拥有的所有资源。)
 * 
 * 如果项目中只有一个线程,panic!将会导致程序报告panic 信息 并退出。
 * 
 * RBE 这本书在讲解Panic！部分貌似有点简单，可以参考其他资料（TRPL）
 * 
 */

<!--
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-20 17:57:41
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-21 15:16:32
 * @FilePath: /RustConqueror/TRR/crates和源文件.md
 * @Description: The Rust Reference 的 Crates and source file章节
-->

在阅读**The Rust Programming Language** 的[Packages and Crates
](https://doc.rust-lang.org/book/ch16-01-threads.html)模块章节时候，彻底被Crates的概念给整懵逼啦,又看了**Rust By Example**，发现两者有些出入，一时拿不定主意。只能来看[The Rust Reference](https://doc.rust-lang.org/reference/crates-and-source-files.html)一探究竟。


Rust属于静态类型语言：分为编译和运行两个阶段。(类似于C,Java)

Rust的编译模型主要是在Crate。每次编译都以源代码的形式处理一个单独的Crate，如果成功，则生成一个独立的二进制格式的Crate：可执行文件或者某种类型的库(either an executable or some sort of library)

Crate是编译，链接，版本控制，分发和运行时加载的单元。一个Crate包含一个嵌套模块作用域的树。模块最顶端的树是匿名的(从模块内路径的角度来看),Crate内的其它内容都有一个规范的模块路径来表示其在Crate中的模块树中的位置。

(Source files in Rust have the extension .rs)
>Rust中源文件指的是就是.rs文件。(源文件这个名词适用于程序员写的程序并保持的文件，C中的.c文件，Java中的.java文件，还有Erlang中的.er文件)

Rust编译器总是用一个源文件作为输入被调用，并且总是产生一个Crate。

### 源文件
> Rust的源文件描述一个模块,(对应于其Crate的模块树)，其名称和位置是定义在源文件的外部。两种方式：
1. 引用源文件的显式模块项项；
2. 通过Crate名称。

每个源文件是一个模块，反之不对。即：模块可以嵌套定义在一个源文件中。

[Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/crates.html):
> A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file.


1. [On the understanding of the concept of crate ](https://github.com/rust-lang/rust-by-example/issues/1374)

2. [How to better understand Crate in Rust?
](https://stackoverflow.com/questions/63515853/how-to-better-understand-crate-in-rust)

综合各方面的信息来理解，Crate 在 Rust的地位比较特殊，范围上要比Module要大。




<!--
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-20 10:13:38
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-20 10:25:05
 * @FilePath: /RustConqueror/TRPL/TRPL.md
 * @Description: 个人经验基于TRPL
-->

> TRPL 即 The Rust Programming Language

1. 数据类型 和 所有权 部分：这本书讲得更好一些；

2. 流程控制 部分：就不要看这本书啦。(建议看RBE的该章节)

3. 最好是先搞明白这本书的『所有权』部分，对于 move，borrow,reference有了一定的理解，再去读RBE的**for-in结构**，会轻松很多。

---
# Understanding Ownership
这部分内容是Rust的核心所在，也是能区别于**程序员人工控制内存**的语言(C,C++ .etc) 以及**内存自动回收机制GC**的语言(Java...)的关键所在。

通过该章，我们才能真正了解：
1. 变量的生命周期；
2. 栈与堆；
3. 变量的clone，move相关内容
4. Tuple和Array,Slice的相关内容
5. String部分内容。
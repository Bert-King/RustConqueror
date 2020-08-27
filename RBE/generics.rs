/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-27 11:18:31
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-27 14:55:34
 * @FilePath: /RustConqueror/RBE/generics.rs
 * @Description: Rust中的泛型
 * 
 * Generics is the topic of generalizing types and functionalities to broader cases. This is extremely
 * useful for reducing code duplication in many ways, but can call for rather involving syntax.
 * 泛型可以将类型和方法泛化到更大的范围。它将在许多方面减少重复代码非常有用。
 * 
 * The simplest and most common use of generics is for type parameters.
 * (泛型最简单和最常见的用法是用于类型参数(type parameters)。)
 * 
 * 泛型参数用<T>来表示。在Rust中，泛型也用来描述接受一个或者多个泛型参数的任何东西。
 * 
 * 
 * Any type specified as a generic type parameter is generic, and everything else is concrete(non-generic).
 * (任何指定为泛型类型参数的类型都是泛型的，其他所有类型都是具体的（非泛型）。)
 */

 // concrete type
 struct A;
// concrete type
 struct Single(A);
// generic type
 struct SingleGen<T>(T);


 fn main(){
     let _s = Single(A);

     let _char:SingleGen<char> = SingleGen('a');

     let _t = SingleGen(A); 
     let _i32 = SingleGen(6);
     let _char = SingleGen('a');
 }

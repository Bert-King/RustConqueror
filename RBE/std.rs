/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 09:57:56
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 10:50:03
 * @FilePath: /RustConqueror/RBE/std.rs
 * @Description: Rust的标准库std
 * >  Box<T> : 建立stack , heap 的桥梁
 * 
 * 
 * 
 * All values in Rust are stack allocted by default.
 * Rust中的所有值默认情况下都是分配在栈上。
 * 
 * Values can be boxed(allocated on the heap) by creating a Box<T>.
 * 通过创建Box<T>将值装箱(分配到堆上).
 * A box is a smart pointer to a heap allocated value of type T.
 * box是类型T的值指向堆内存的智能指针。
 * When a box goes out of scope, its destructor is called, the inner object is destroyed, and the momery on the heap is 
 * freed.
 * 当box超出其作用域，它的解构被调用，其内部对象将会被销毁，同时堆内存也会被释放。
 * 
 * Boxed value can be dereferenced using the * operator;this removess one layer of indirection(间接).
 * 被装箱的值通过*操作符，可以被间接引用。
 * 
 * 简而言之: Box装箱操作: 
 * 1. Box::new(t) -> Box<T>
 * 2. box的size就是pointer的size.
 * 3. *操作符进行拆箱操作。
 * --------------------------------
 * std::mem模块可以用来显示变量所占用的内存大小。mem::size_of_val(&v)
 * 
 */

 use std::mem;

 /**
  * 点
  */
 struct Point{
     x:f64,
     y:f64,
 }

 /**
  * 矩形
  */
 struct Rectangle{
     top_left:Point,
     bottom_right:Point,
 }

/**
 * 原点(分配在Stack)
 */
 fn origin() -> Point{
     Point{x:0.0,y:0.0}
 }

 /**
  * 分配在Heap上，并返回一个pointer
  */
 fn boxed_origin() -> Box<Point>{
     Box::new(origin())
 }

 /**
  * 分配在Stack的矩形
  */
fn rectangle() -> Rectangle {
    Rectangle{
        top_left:origin(),
        bottom_right: Point{x: 5.0,y:5.0}
    }
}
/**
 * 分配在Heap的矩形
 */
fn boxed_rectangle() -> Box<Rectangle> {
    Box::new(rectangle())
}


fn main(){

    let origin = origin();
    let rectangle = rectangle();

    println!("Point occupies {} bytes on the stack ", mem::size_of_val(&origin));
    println!("Rectangle occupies {} bytes on the stack ", mem::size_of_val(&rectangle));
    println!("-------------------------------------------------");

    // box size == pointer size
    let boxed_origin = boxed_origin();
    let boxed_rectangle = boxed_rectangle();
    println!("Boxed point occupies {} bytes on the stack ", mem::size_of_val(&boxed_origin));
    println!("Boxed rectangle occupies {} bytes on the stack ", mem::size_of_val(&boxed_rectangle));

    // 使用*操作符进行拆箱(有点C(++)的味道...)
    let unboxed_rectangle:Rectangle = *boxed_rectangle;
    println!("Unboxed rectangle occupies {} bytes on the stack ", mem::size_of_val(&unboxed_rectangle));

    let double_boxed_origin:Box<Box<Point>> = Box::new(boxed_origin);
    println!("Double boxed point occupies {} bytes on the stack ", mem::size_of_val(&double_boxed_origin));


}


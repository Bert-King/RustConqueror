/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 10:52:36
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 14:03:00
 * @FilePath: /RustConqueror/RBE/std_vectors.rs
 * @Description: 标准库中的Vector
 * 
 * Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they can grow or shrink at any time. 
 * (向量是动态数组，就像Slice一样，它的size在编译期间是不知道的，但是可以在任意时间增长或者缩小。)
 * 
 * Rust的Vector:类似于C++的Vector，Java中的ArrayList
 */

 fn main(){
     let collected_iterator:Vec<i32> =(0..10).collect();
     println!("Collected (0..10) into: {:?}",collected_iterator);

     let mut xs = vec![1i32,2,3];
     println!("Initial vector:{:?}",xs);
     xs.push(4);
     println!("Vector:{:?}",xs);
     println!("Vector length:{:?}",xs.len());


     // 访问Vector元素的两种方式
     println!("First element:{}",xs[0]);
     println!("First element:{:?}",xs.get(0));

     println!("Pop last element:{:?}",xs.pop());

    // println!("{}",xs[100]); // index out of bounds

    // 遍历
    for x in xs.iter(){
        print!("{} ",x);
    }

    println!();
    /**
     * 带下标的遍历
     */
    for (i,x) in xs.iter().enumerate(){
        println!("In position {} we have value {}",i,x);
    }

    /**
     * 由于item_mut()。我们可以在遍历过程中修改元素
     */
    for x in xs.iter_mut(){
        *x *= 2;
    }
    println!("Updated vector:{:?}",xs);


 }

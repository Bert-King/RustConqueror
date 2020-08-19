/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 14:21:15
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-19 16:05:23
 * @FilePath: /RustConqueror/RBE/control_flow_for_and_range.rs
 * @Description: for循环
 * 
 * In Rust, 采用的是for-in结构。（Python，Kotlin等语言也是采用这种方式，而Java，C等却是i++结构）
 * 
 * The for in construct can be used to iterate through an Iterator
 * (for-in结构可以被用来遍历迭代器。关于Iterator的概念在其它语言已经不算是什么秘密啦。)
 * 
 * Iterator:迭代器用于在集合(如：array)上实现迭代功能。
 * 简而言之，就是将集合转化为迭代器，使之可以被迭代。
 * As a point of convenience for common situations, the for construct turns some collections into iterators using the .into_iter() method.
 * 为了方便常见情况，for结构使用.into_iter()方法将一些集合转化为迭代器。
 * 
 * 
 * 
 * 
 * Rust中创建迭代器的最简单方式之一是：range。
 * > 1. a..b (from a (inclusive) to b (exclusive) in steps of one) 每次+1，从a都b-1
 * > 2. a..=b ：从a 到 b. 
 * 貌似这里不能设置step.有点搞不懂为啥。
 * 
 * Rust中for-in结构的独特之处:
 * The for-in construct is able to interact with an Iterator in serveral ways.
 * for-in 机构有多种方式与Iterator进行交互。
 * 
 * 1. into_iter（for-in结构的默认方式）
 * 2. iter
 * 3. iter_mut
 * 这三种方式都可以处理从集合到迭代器之间的转化。
 * 
 * 看了这三种方式，总感觉默认采用into_iter不太合理，有木有啊？
 * 
 * 
 */

 fn main(){
    for_range();

    for_iter();

    for_into_iter();

    for_iter_mut();
 }

 /**
  * 通过range notation来创建Iterator.
  */
 fn for_range(){
    for n in 1..11{
        println!("{}",n);
    }
    println!("use a..=b as below");
    for n in 1..=10{
        println!("{}",n);
    }
 }


 /**
  * iter: This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
  * 通过每次迭代借用集合的每个元素，因此使集合保持不变，并在循环后可重用。
  * (特点: 集合不变，可重用)
  */
 fn for_iter(){
     let levels = vec!["First","Second","Third"];
     
     for level in levels.iter() {
         match level {
             &"First" => println!("The value is 1"),
             _ => println!("The index is {}",level),
         }
     }

     println!("levels is {:?}",levels);
 }

/**
 * into_iter : This consumes the collection so that on each iteration the exact data is provided. 
 * Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
 * (这种方式将消耗集合以便在每次迭代过程中提供精确的数据。一旦集合被消耗，它将不再可重用，因为在循环中已经被移动。)
 * 
 * (特点：集合移动，不可重用)
 */
 fn for_into_iter(){
    let levels = vec!["First","Second","Third"];
    
    for level in levels.into_iter() {
        match level {
            "First" => println!("The value is 1"), // 注意这里没有&
            _ => println!("The index is {}",level),
        }
    }

    //println!("levels is {:?}",levels);// Error: value borrowed here after move
}

/**
 * iter_mut : This mutably borrows each element of the collection, allowing for the collection to be modified in place.
 * (特点：可修改集合)
 */
fn for_iter_mut(){
    let mut levels = vec!["First","Second","Third"];
    
    for level in levels.iter_mut() {
      *level =   match level {
            &mut "First" => "1",
            &mut "Second" => "2",
            _ => "3", 
        }
    }
    println!("levels is {:?}",levels);
}
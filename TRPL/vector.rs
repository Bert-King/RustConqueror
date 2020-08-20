/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-20 11:37:42
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-20 15:06:59
 * @FilePath: /RustConqueror/TRPL/vector.rs
 * @Description: Rust中的动态数组Vector (C++的朋友应该对此感到非常亲切吧)
 * 
 * 尖括号：angle bracket
 * 
 * 
 * 世界可并不总是那么完美，很多情况下，我们是没法来预料内容的大小的。(正如你不知道你有多么富有一样 ~2333~)
 * A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. 
 * 
 * If you’re unsure whether to use an array or a vector, you should probably use a vector.
 * 
 * 显然Rust为了避免不得不浪费空间的做法，而选择了动态数组。（C++，Java都是这么干的，没什么新奇的）。
 * 
 * Rust的Vector的特点：
 * 1. 内存分配在堆上；
 * 2. 相较于数组，更加灵活，其size可动态调整。
 * 
 * ------
 * 老规矩，先看一下如何创建向量Vector（从无到有的过程）
 * 1. Vec:new();
 * 2. vec![] ; 
 * 
 * 访问Vector中的元素，有两种方式：
 * 1. &V[index]; 返回对应下标的具体元素。有可能出现下标越界 
 * 2. V.get(index);返回一个Some(&element) Or None(下标越界时)
 * (关于所有权borrow部分的内容，需要等完全吃透所有权再补充)
 * 
 * 
 * Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. 
 * 
 * 取消引用操作符*
 */
fn main() {
    
    update_vecotr();

    read_element_vector();

    iterate_vector();

    iterate_change_vector();
}

/**
 * 创建Vector的两种方式
 */
fn create_vector(){
    // 使用标准库的Vec::new();
    let v:Vec<isize> = Vec::new();

    // 使用宏vec!
    let numbers = vec![1,2,3,4,5];
}

/**
 * 通过push添加元素
 * 
 * When the vector gets dropped, all of its contents are also dropped。
 * (当Vector被清除，其内部的内容也跟着被消除)
 */
fn update_vecotr(){
    let mut container = Vec::new();//只有添加mut才可以update
    
    container.push(100);
    container.push(200);
    container.push(300);
    
    for i in &container {
        println!("The value is {}",i);
    }
}

/**
 * Now that you know how to create, update, and destroy vectors, knowing how to read their contents is a good next step. 
 * 两种访问Vector元素的方式:
 * 1. &V[index]; 返回对应下标的具体元素。有可能出现下标越界
 * 2. V.get(index);返回一个Some(&element) Or None(下标越界时)
 */
fn read_element_vector(){

    let odds = vec![1,3,5,7,9];

    println!("The third element is {}", &odds[2] ); // 5

    println!("The First element is {:?}", odds.get(0) ); // Some(1)

    println!("The absent element is {:?}", odds.get(10) ); // None

    // println!("The absent element is {}", &odds[10] ); // index out of bounds

}

/**
 * 普通遍历
 */
fn iterate_vector(){
    let odds = vec![1,3,5,7,9];

    for i in &odds {
        print!(" {} ",i);
    }
    println!("");
}

/**
 * 这里使用『取消引用运算符*』获取引用的值（C的感觉...）
 */
fn iterate_change_vector(){
    println!("Evens as follow...");
    let mut odds = vec![1,3,5,7,9];
    let mut evens = for i in &mut odds {
        *i += 1;
        print!(" {}, ", i);
     };
}


/**
 * 使用枚举来增强Vetor,使其支持多种类型。(这有点类似Java中的List放Bean啊。。。)
 * Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. 
 * A secondary advantage is that we can be explicit about what types are allowed in this vector. 
 */
fn multiple_type_vecotr(){

    enum Person{
        Age(i32),
        Address(&str),
        Weight(f32),
        Gender(bool),
    }

    let infos = vec![Person::Age(25),
                    Person::Address("LuoYang"),
                    Person::Weight(75.5),
                    Person::Gender(true),
                ];

}
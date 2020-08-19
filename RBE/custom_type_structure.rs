/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-19 16:34:49
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-19 17:16:27
 * @FilePath: /RustConqueror/RBE/custom_type_structure.rs
 * @Description: Rust中使用关键字struct 可以创建3种类型的结构体
 * 
 * 1. Tuple structs(元组结构，即：可命名的元组)
 * 2. 传统的C系列结构体。(有点类似于面向对象语言的类)
 * 3. Unit stucts(空结构体，即没有字段，这类结构体对后面的泛型非常有用)
 * 
 * 需要注意的是：定义或者初始化结构体，使用的是{}
 * 
 */


 // Tuple Struct
 struct Pair(isize,isize);

 // Unit struct
 struct Unit;

 #[derive(Debug)]
 struct Person<'a>{
     // 'a defines a lifetime (后面再讲这是什么东西)
     name: &'a str,
     age: u8,
     address: &'a str,
     gender: bool,
 }

 // A struct with two fields.
 struct Point {
    x:f64,
    y:f64,
 }

 // Structs can  be reused as fields of another struct(结构体可以被复用为其它结构体的变量)
 struct Line {
     start:Point,
     end:Point,
 }

 fn main(){
     let name = "Bertking";
     let age = 25;
     let address = "LuoYang";
     let gender = true;

     let owner = Person{ name,age,address,gender };

     println!("{:?}",owner);


     let point:Point = Point{x:0.5,y:0.5};
     println!("x = {}, y = {}",point.x,point.y );


     let line:Line = Line{ start:Point{x:0.0,y:0.0}, end: Point{x:1.0,y:1.0} };


     let _unit = Unit;

     let pair = Pair(6,9);
     println!("first is {:?},second is {:?}", pair.0 , pair.1 );

     // Destructure 解构 。这在Erlang种被称为『模式匹配』
     let Pair(key,value) = pair;
     println!("key is {:?},value is {:?} ",key,value);

 }
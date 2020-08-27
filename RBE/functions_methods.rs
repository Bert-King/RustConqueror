/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-27 15:23:42
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-27 16:30:35
 * @FilePath: /RustConqueror/RBE/functions_methods.rs
 * @Description: Rust中函数的分类:Method
 * 
 * Methods are functions attached to objects. These methods have access to the data of the object and its other methods via the self keyword.
 * Methods are defined under an impl block.
 * (Methods 定义在impl代码块中.Methods 其实就是绑定在对象上的函数,这些methods 可以访问对象数据以及通过关键字self访问其它方法)
 * 
 */

 struct Point{
     x:f64,
     y:f64,
 }


 /**
  * Implementation block, all `Point` methods go in here
  * Point 的实现代码块，所有Point相关的Methods都定义在这里
  */
 impl Point{
    /**
     * 静态方法(static method)
     */
    fn origin() -> Point{
        Point{x:0.0,y:0.0}
    }
    
    /**
     * 静态方法
     */
    fn new(x:f64,y:f64) -> Point{
        Point{x:x ,y:y}
    }

 }



 struct Rectangle{
     p1:Point,
     p2:Point,
 }


 impl Rectangle{
     /**
      * 实例方法(instance method)
      * &self 是 self:&Self的语法糖。这里的Self就是调用对象的类型。
      * 这里：Self = Rectangle. 
      */
     fn area(&self) -> f64 {
         let Point{x : x1, y :y1} = self.p1;
         let Point{x : x2, y :y2} = self.p2;

         ((x2-x1)*(y2-y1)).abs()
     }

     fn perimeter(&self) -> f64 {
         let Point{x:x1,y:y1} = self.p1;
         let Point{x:x2,y:y2} = self.p2;

         2.0 *((x2-x1).abs() + (y2-y1).abs())
     }

     /**
      * 要求是:可变的对象
      * &mut self 本质上是 self:&mut Self.
      */
     fn translate(&mut self,x:f64,y:f64){
         self.p1.x += x;
         self.p2.x += x;

         self.p1.y += y;
         self.p2.y += y;
     }

 }

 struct Pair(Box<i32>,Box<i32>);

 impl Pair{
    /**
     * self 是 self:Self 
     * 这样调用此方法只能调用一次，因为该方法将会消耗掉调用对象的资源。
     */
    fn destroy(self){
        let Pair(first,second) = self;
        println!("Destroying Pair({},{})",first,second);
    }

 }
 fn main(){
     let rectangle = Rectangle{
         // 使用(double colons)::来访问静态方法
         p1:Point::origin(),
         p2:Point::new(3.0,4.0),
     };


     /**
      * 使用(dot).操作符来调用实例方法
      * 注意这里：rectangle.perimeter() 等价于 Rectangele::perimeter(&rectangle).
      */
     println!("Rectangle perimeter:{}",rectangle.perimeter());
     println!("Rectangele area:{}",rectangle.area());



     let mut square = Rectangle{
         p1:Point::origin(),
         p2:Point::new(1.0,1.0),
     };
    //cannot borrow `rectangle` as mutable, as it is not declared as mutable
    // rectangle.translate(1.0, 1.0); 

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1),Box::new(2));
    pair.destroy();

   // pair.destroy();





 }
/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-20 16:02:53
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-21 17:34:21
 * @FilePath: /RustConqueror/RBE/module_and_visibility.rs
 * @Description: Rust的模块(感觉跟JS的差不多,跟Java相去甚远)
 * 
 * 模块这个概念，对于有其他经验的开发者绝对不会陌生。
 * 
 * Rust中的模块是:functions, structs, traits,impl blocks and other modules 的集合。
 * 在Rust中，一个项目按照范围的大小分为：
 * Packages ---> Crates ---> Modules(说实话看TRPL的这部分内容成功把我搞懵逼...)
 * 这里对于Crate补充一点：
 *  RBE上面的说明是完全符合Rust文档的。什么是Crate?
 * --------------------------------------------
 * 
 * 模块的主要作用：
 * 1. 控制作用域的范围；
 * 2. 访问权限控制；
 * 
 * 使用mod来显式地声明一个Module。
 * 需要注意的是:一个.rs文件默认情况下有一个匿名的Module。
 * 就像我们的main()函数其实就是定义在其中的
 * 
 * Module的可见性规则:
 * 1. 所有的items默认都是私有的(private visibility)；
 * 2. 可以通过修饰符pub来修改items的可见性。
 * 3. 只有public的items才能在module的作用域以外被访问。
 * 4. module也是可以被嵌套的。
 * 5. 同一module下那自然是不受visibility的约束。
 * 
 * 有其它编程经验的开发者，对此应该没有任何障碍。
 * 
 */
 mod my_mod{
     /**
      * Module中的items默认都是private visibility.模块外部访问不了的。
      */
     fn private_func_in_my_mod(){
         println!("Called my_mod::private_function()");
     }

     /**
      * 使用pub修饰符，将其声明为public visibility.
      */
     pub fn  public_func_in_my_mod(){
        println!("Called my_mod :: function()");
     }

     /**
      * 同一模块下的items将不受visibility的控制.
      */
     pub fn indirect_access(){
        println!("Called  my_mod ::indirect_access()");
        private_func_in_my_mod();
     }

     /**
      * 嵌套的Module（有没有内部类的感觉）
      * 注意这里用的是pub修饰，
      */
     pub mod public_nested{
        pub fn public_func_in_public_nested_mod(){
            println!("Public function in public nested mod", );
        }
        #[allow(dead_code)]
        fn private_func_in_public_nested_mod(){
            println!("Private function in public nested mod")
        }
        /**
         * pub(crate)使该方法仅仅对当前的crate可见。
         */
        pub(crate) fn public_func_in_crate(){
            println!("Just only be called in this crate(注意这里是Crate)")
        }

     }

     /**
      * 嵌套的私有module
      * visibility 适用于 modules.
      */
    #[allow(dead_code)]
     mod private_nested{
         pub fn public_func_in_private_nested_mod(){
            println!("Public function in private nested mod", );
         }

         fn private_func_in_private_nested_mod(){
            println!("Private function in private nested mod")
        }
     }


 }


 fn main(){
     my_mod::public_func_in_my_mod();

     my_mod::indirect_access();

     my_mod::public_nested::public_func_in_public_nested_mod();
 }
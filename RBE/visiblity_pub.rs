/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-21 15:19:13
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-21 17:23:28
 * @FilePath: /RustConqueror/RBE/visiblity_pub.rs
 * @Description: Rust中的Module所有的items默认都是private visibility
 * 
 * 我们可以通过修饰符pub来修改visibility.
 * 我们知道这其中有很多种情况，Rust仅仅靠pub和默认private肯定是不够的。
 * 
 * 这里着重来看一些特殊情况：
 * 
 * pub
 * pub(self):当前模块可见，等价于默认的private(鸡肋)
 * pub(super):父模块可见
 * pub(crate): 当前Crate可见
 * pub(in path):当前路径可见(path必须是父or祖先模块)
 * 
 */

 mod outer_mod{

    pub fn outer_func(){
        inner_mod_one::inner_one_func_super();

        // error[E0603]: function `inner_one_func_self` is private
       // inner_mod_one::inner_one_func_self();

        // pub(in path) items can only be called from within the module specified
       inner_mod_one::inner_one_func_in_path();
    }

    pub mod inner_mod_one {
        pub fn inner_one_func(){
            println!("Public function in module1 of outer");
            inner_one_func_super();
        } 

        /**
         * 使用pub(self) 等价于 private，即仅限于当前模块可见。
         * (意义何在？除了增加出错的几率)
         */
        #[allow(dead_code)]
        pub(self) fn inner_one_func_self(){
            println!("Just visible for current module...");
        }

        /**
         * pub(super) 只对当前父模块可见(即：只能在父模块中使用)
         * 比较奇葩的是---不能这样调用:outer_mod::inner_mod_one::inner_one_func_super()
         * 只能间接地活跃在父模块中。即::符号是不具备传递功能的。
         */
        pub(super) fn inner_one_func_super(){
            println!("Just visible of parent module...");
        }

        /**
         * pub(in path) 仅对给定的路径可见。path必须是父或者祖先模块(parent or ancestor module)
         */
        pub(in crate::outer_mod) fn inner_one_func_in_path(){
            println!("Functions declared using pub(in path)...");
        }

        /**
         * put(crate)仅仅在当前的Crate可见
         */
        pub(crate) fn inner_mod_one_crate(){
            println!("Functions declared using pub(crate)...");
        }

    }

    /**
     * 同级别的模块无法相互访问吗？(有待深究)
     */
    #[allow(dead_code)]
    pub mod inner_mod_two{
        pub fn inner_two_func(){
            println!("Public function in module2 of outer");
           // outer_mod::inner_mod_one::inner_one_func();
        } 
    }


 }


 fn main(){
     outer_mod::inner_mod_one::inner_one_func();

     // error: function `inner_one_func_super` is private
   // outer_mod::inner_mod_one::inner_one_func_super();

     outer_mod::outer_func();
    
     // error[E0603]: function `inner_one_func_in_path` is private
    // outer_mod::inner_mod_one::inner_one_func_in_path();
 }

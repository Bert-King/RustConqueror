
/**
 * To show Rust where to find an item in a module tree.(path)
 * 
 * A path can take two forms:
 * 
 * 1. An absolute path starts from a crate root by using a crate name or a literal crate.
 * （以crate文件的名字或者关键字crate开头的绝对路径）
 * 2. A relative path starts from the current module and uses self,super, or an identifier in the current module.
 *  (以当前模块或者使用selft,super以及当前模块标识符开头的相对路径)
 * 
 * 绝对路径和相对路径后面都有一个or多个标识符，用双冒号(::)分隔。
 * 
 * 总结：推荐使用super，self关键字将会在修改crate模块树的情况下，尽可能少的改动代码。
 * 
 * 模块系统的根节点就是crate.
 * 
 * ----------------------------------------------------
 * 如果有JavaScript, Python的经验，则后面这部分内容可以说是0障碍。
 * -----------------------------------------------------
 * 关键字use : 将路径(相对路径&绝对路径)引入到作用域内。(引入标准库的HashMap结构体:use std::collections:HashMap;)
 * 
 * 使用path上惯例:
 * 1. 调用函数时不建议使用全路径。(分不清本地函数还是外部函数)
 * 2. 引入结构体，枚举以及其他items时，一般都指定全路径。
 * 3. 通过as关键字给path起别名。
 * 4. pub use 被称为：re-exporting
 * 5. 如果引入外部库，我们需要在Cargo.toml 的 dependencies 告诉Cargo进行下载，然后use引入即可使用
 * 需要注意的是，标准库(std)在我们的包中也是一个Crate。因为标准库随着Rust一起被提供，所以我们无需在Cargo.toml中引入。
 * 6. 路径嵌套：use std::{collections::HashMap,io}; 注意自身用self标识。
 * 7. use std::collections::* ; (* 通配符)
 * 
 * 
 * There's no strong reason behind this idiom: it's just the convention that has emerged, and folks have gotten used to
 * reading and writing Rust code this way.
 * (这个习惯用法背后没有什么强有力的理由: 它只是出现了一种惯例，人们已经习惯了用这种方式读写Rust代码。)
 * 
 */

 use std::fmt::Result;

 use std::io::Result as IoResult;



//  use std::io;
//  use std::io::Write;

 // 上述可以被简化
 use std::io::{self,Write};











mod front_of_house{
    pub mod hosting{
       pub fn add_to_waitlist(){}

       pub fn seat_at_table() {}
    }

   pub mod serving{
      pub  fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

fn serve_order(){}

mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast{
        pub fn summer(toast:&str) -> Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }


    pub enum Appetizer{
        Soup,
        Salad,
    }


    fn fix_incorrent_order(){
        cook_order();
        super::serve_order();// 此处的super，指的是crate(the root)
    }

    fn cook_order(){}
}


use self::front_of_house::hosting;
use crate::front_of_house::serving;

use crate::front_of_house::serving::take_order;

use std::collections::HashMap;





pub fn eat_at_restaurant(){
    // 绝对路径(Absolute path)
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径(Relative path) slef or super在path只能用在开头位置
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    serving::take_order();

    take_order();// 不推荐

    let mut map = HashMap::new();
    map.insert("R", 1);


    
}

/**
 * 从这里我们可以看出：.rs文件默认就是一个模块。
 */
pub mod car;
pub mod bmw;
pub use crate::car::benzi;



fn main(){
    car::benzi::drive();

    bmw::drive();
}


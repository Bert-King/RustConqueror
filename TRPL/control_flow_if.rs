/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-18 19:37:17
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-18 20:16:03
 * @FilePath: /RustConqueror/TRPL/control_flow_if.rs
 * @Description: Rust中的条件控制语句： 
 * 
 *  1. if...else...  
 *  2. if...else if...else...
 *  3. match
 */

fn main(){
    if_expressions();

    get_week(5);

    get_week_match(8);

    if_let_statement();

}

/**
 * 和其他语言一样，if 表达式 必须是bool类型。
 * 
 * It's really worth noting that...（但是Rust中的if表达式不能被小括号包裹，真的有点不适应）
 */ 
fn if_expressions(){
    let is_true = false;
    if is_true {
        println!("True");
    } else {
        println!("false");
    }
}

/*
 * 使用if 表达式处理多条件的情况。
 * 作为编程老手的我们，对于这种多个else if表达式的写法肯定是嗤之以鼻。因为这种情况往往意味着我们需要重构代码啦。
 * Rust作为站在巨人肩膀上的语言，早已为我们提供了这种情况的解法：match（类似于Erlang中的模式匹配）
 * 
*/
fn get_week(day:usize){
    if day == 1{
        println!("Today is {}",day );
    } else if day == 2 {
        println!("Today is {}",day );
    } else if day == 3 {
        println!("Today is {}",day );
    } else if day == 4 {
        println!("Today is {}",day );
    } else if day == 5 {
        println!("Today is {}",day );
    }else if day == 6 {
        println!("Today is {}",day );
    }else {
        println!("Today is {}",day );
    }
}

/**
 *  使用match替换多个else if.(注意这里的剩余情况使用占位符_来匹配的，类似于Java中的switch中default) 
 *  处处透露着Erlang的影子
 */ 
fn get_week_match(day:usize){
    match day {
        1 => {
            println!("Today is {}",day );
        }

        2 => {
            println!("Today is {}",day );
        }

        3 => {
            println!("Today is {}",day );
        }

        4 => {
            println!("Today is {}",day );
        }

        5 => {
            println!("Today is {}",day );
        }

        6 => {
            println!("Today is {}",day );
        }

        _ => {
            println!("Today is {}",day );
        }
    }
}

/**
 * 因为if是表达式，所以我们可以用在let语句的右边。（类似于Java或者C的?:） 
 * 唯一需要注意的是: if 和 else中的类型要相同
 */
fn if_let_statement(){
    let isMan = true;
    let gender = if isMan {"Man"} else {"Woman"};
    println!("Gender = {}",gender );
}
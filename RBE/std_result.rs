/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-26 14:23:05
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-26 15:44:01
 * @FilePath: /RustConqueror/RBE/std_result.rs
 * @Description: Rust中的枚举 Result
 * 
 * 发展历程:Option<T> ---> Result<T,E> ---> 操作符？
 * 
 * 我们使用Option<T>作为返回值时，发生错误将会返回None。
 * 但是如果我们需要知道出错的详细信息，这时我们就可以采用枚举Result
 * 
 * Result<T,E> enum含有两个变体(variants):
 * 1. Ok(value):表示操作成功。(value:T 返回结果)
 * 2. Err(why): 表示操作失败。(why:E 解释错误)
 * 
 * 通过下面的例子我们会发现，使用match来得到结果显得复杂混乱,操作符？可以使事情变得更好。
 * 表达式末尾添加？ === match表达式
 * 
 */
 mod checked{

    pub enum MathError{
        // 除0
        DivisionByZero,
        // 对数非正
        NonPositiveLogarithm,
        // 负数开方
        NegativeSquareRoot,
    }

    // 这里的type是什么鬼？
    pub type MathResult = Result<f64,MathError>;


    /**
     * 除法
     */
    pub fn div(x:f64,y:f64) -> MathResult{
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else{
            Ok(x/y)
        }
    }


    /**
     * 开方
     */
    pub fn sqrt(x:f64) -> MathResult{
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        }else{
            Ok(x.sqrt())
        }
    }

    pub fn ln(x:f64) -> MathResult{
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        }else{
            Ok(x.ln())
        }
    }

 }


 fn op(x:f64,y:f64) -> f64{
    match checked::div(x, y) {
        Err(why) => panic!("{:?}",why),
        Ok(ratio) => ratio,
    }
}
/**
 * 改善版本
 */
fn op_pretty(x:f64,y:f64) -> MathRsult{
    let radio = div(x,y)?;

    let ln = ln(radio)?;
}








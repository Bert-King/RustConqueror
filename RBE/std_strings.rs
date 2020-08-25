/*
 * @Author: BertKing
 * @version: 
 * @Date: 2020-08-25 14:03:39
 * @LastEditors: BertKing
 * @LastEditTime: 2020-08-25 15:15:55
 * @FilePath: /RustConqueror/RBE/std_strings.rs
 * @Description: Rust中的字符串
 * 
 * 值得我们注意的是：Rust中存在两种类型的字符串:String 和 &str.
 * 
 * A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. 
 * String is heap allocated, growable and not null terminated.
 * (String 存储为字节向量，但保证始终是有效的UTF-8序列。String是堆分配的，可增长的并不以null结尾。)
 * 
 * &str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.
 * (&str是数组u8的切片并一直指向有效的UTF-8序列，可以用来查看字符串。就像&[T]是Vec<T>的视图一样)
 * 
 * 同其它编程语言一样，Rust同样面临着涉及到转义字符的问题：
 * 1. 如果转义字符很少的话，我们就可以使用其它语言类似的技巧去处理。
 * 2. 如果转义字符太多的话，可以借助于r"..." 或者r##"..."##的处理方式。后面的#数量没有限制，但是必须和后面的配对。
 * 如果字符串中含有双引号，则使用r##"..."...".."##.
 * 如果字符串中含有星号*,则使用r###"..."...#..."..."###即可。
 * 
 * 3. b"" 可以将字符串转化为字节数组
 * 
 * 另外：&str 和 String 都必须是有效的UTF-8.
 * 
 * 建议：要想真正的理解两者的区别，需要首先知道：Vector和Slice的区别。
 * 
 */

 fn main(){

    // A reference to a string allocated in read only memory
    let pangram:&'static str = "some thing different between string and &str";
    println!("Pangram:{}",pangram);

    println!("反向遍历...");
    for word in pangram.split_whitespace().rev(){
        println!("> {}",word);
    }

    let mut chars:Vec<char> = pangram.chars().collect();
    println!("原数据:{:?}",chars);
    chars.sort();
    println!("排序后:{:?}",chars);
    chars.dedup();
    println!("去重后:{:?}",chars);

    println!("-------------------------");

    // Create an empty and growable 'String'
    let mut string =String::new();

    for c in chars{
        string.push(c);
        string.push_str(",")
    }

    println!("{}",string);


    let chars_to_trim:&[char] = &[' ', ','];
    // trimmed string 是 string的切片，因此运行时不会重新分配内存
    let trimmed_str:&str = string.trim_matches(chars_to_trim);
    println!("Used characters:{}",trimmed_str);

    // 堆分配的String
    let alice = String::from("I like dogs...");
    // 分配新的内存存储修改的String
    let bob:String = alice.replace("dog", "tiger");

    println!("Alice says:{} ",alice);
    println!("Bob says:{}",bob);

    println!("Rust字符串涉及到转义字符的场景:");

    let raw_str = r"Escape don't work here:\x3F";
    println!("{}",raw_str);

    let quotes = r#"And then I said:"There is no escape!""#;
    println!("r#适用于引号的场景:{}",quotes);

    let longer_delimiter = r####"A string with "# in it. And even "##!"####;
    println!("如果字符串中含有#，则使用更多的#即可：{}",longer_delimiter);

    // b"" 可以将字符串转化为字节数组
    let bytestring = b"012345";
    println!("A byte string :{:?}",bytestring);



 }

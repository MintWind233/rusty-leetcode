# rusty-leetcode

使用Rust刷Leetcode的练习，以及总结了一个用来刷算法的文档

对应链表写在Linked List，  
对应树的题写在Tree,  
对应动态规划题目写在Dynamic Programming,  

对应不是分项目的分别的题号为lc-xxxx

```shell
cd lc-xxxx
cargo run
```

### Rust刷算法题常用方法手册

本手册着力于解决大家使用Rust写算法题，经常找不到对应方法的窘境，查std文档也是一个效果，不过std文档是英文的，我负责给出一些短平快的解决方案给大家

1. HashSet 相关操作

2. String相关操作

   ```rust
   
   // 创建字符串
   let mut s = String::from("Hello,World");
   
   // 字符串当中查找元素的下标
   assert_eq!(s.find('o').unwrap(), 4);// 返回 Option<usize> 类型
   
   // 字符串中查找是否含有某一元素
   assert_eq!(s.contains('o'),true);
   
   // 根据下标获取字符串当中的元素
   assert_eq!(&s[1],'e'); // 第一种方式返回 &str 类型
   
   // 转大写
   assert_eq!(s.to_uppercase(), "HELLO,WORLD".to_string());
   
   // 连接
   assert_eq!(format!("'{}'",s),"'Hello,World'");
   
   // 遍历
   for chr in s.chars() {};
   for byte in s.bytes() {};
   ```

3. HashMap哈希表相关操作

   ```rust
   let mut mp:HashMap = HashMap::new();
   
   // 查询key对应的值，若不存在则插入新值，若已经存在则进行更新
   let count = mp.entry(word).or_insert(0);
   *count += 1;
   
   // 转换为数组
   let vec_key = mp.into_keys().collect();
   let vec_value = mp.into_values().collect();
   
   // 遍历
   for (key, val) in mp.iter() {};
   
   // 可变遍历
   for (key, val) in mp.iter_mut() {};
   ```

4. Vec数组相关操作

   ```rust
   use std::collections::Vec;
   // 创建Vec数组
   let vec = vec![];
   
   // vec转iter
   vec.into_iter(); // 获取所有权的迭代器
   vec.iter(); // 借用迭代
   vec.iter_mut(); // 可变借用迭代器
   
   // vec数组求和
   let sum = vec.into_iter().sum();
   
   // vec数组转其他类型
   let set:HashSet<i32> = vec.into_iter().collect()
   ```

5. 查看内存地址

   ```rust
   let var:i32 = 3;
   let ptr = var.as_ptr();
   println!("{}",ptr);
   println!("{:p}",&var);
   ```

6. Option相关操作

   ```rust
   // 创建Option
   let var:Option<Rc<3>>;
   
   // 查询Option是否存在
   var.is_none();
   ```

7. 双端队列

   ```rust
   
   ```

8. 读取文件

   ```rust
   let input: String = std::fs::read_to_string("path")?;
   ```

9. 逐位对比

   ```rust
   // 假设我想对比字符串str1和str2
   let mut ctx = str1.chars().zip(str2.chars())
   assert_eq!(ctx.next(),('f','z'))
   
   for (char1, char2) in str1.chars().zip(str2.chars()) {}
   ```

10. 213

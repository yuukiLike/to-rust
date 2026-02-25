// 好，String 和 &str 的基础你已经掌握了。再补一个常用知识点：两者之间怎么互转。                                                                         
                                                                                                                                                        
//   // String → &str：加 & 就行                                                                                                                           
//   let s: String = String::from("hello");                                                                                                                
//   let r: &str = &s;                                                                                                                                     
                                                                                                                                                        
//   // &str → String：两种写法都行                                                                                                                        
//   let r: &str = "hello";                                                                                                                                
//   let s1: String = r.to_string();                           
//   let s2: String = String::from(r);

//   记忆：借用是"轻"操作（加个 &），获得所有权是"重"操作（要分配堆内存，所以需要调方法）。

//   ---
//   任务：写一个函数 shout(s: &str) -> String，把传入的字符串转成全大写返回。

//   提示：s.to_uppercase() 返回一个 String。

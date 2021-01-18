how to use:  

# How to structure a rdb file: 

```
|test_A|This is an array ,b,c,1ad  
|test_M|This is a string or Meta tag   
|test_D|this is a number tag  
```
add _A or _M or _D to the name to select the types!  
the _A _M and _D will be ignored when using findValue  
so only use the name you specified when trying to look for it.  

## How to 'compile' a rdb file  

use rdbData(&str) the string is the name of the file.  
to find the position of the data use findValue  
Disclaimer: due to rust for some reason the values will start on 1 and not 0.  
if you try to get value 0 you will only be left with "" or 0  
  
after that you will be given a dataBook ( from rdbData):  
```rust
pub struct dataBook{  
    pub NM:Vec<String>, //NM = name meta  
    pub ND:Vec<String>, //ND = name digit  
    pub D:Vec<i64>, //D = digit  
    pub M:Vec<String>, //M  = meta  
    pub A:Vec<Vec<String>>, //A = array  
    pub AN:Vec<String> //AN = array name  
}
```
you dont need to make your own dataBook as it can be found in rustDatabook::dataBook  
  
## How to use find value  

 findValue(&str,&dataBook,&str) -> usize first str is the name of the value and the second str is what type so _A _M _D  

#3 How to change data:

changeData(&str &str &str): name of the file, the new data, the name of the data aka |changethisdata_M|

## How to add Data:

addData(&str &str): name of the file, the new data in this format |name+(type)|added data

## Remove data:

removeData(&str,&str): name of the file, name of the data

after changeing  data you need to re parse the file
  
## How to use copyValueToNew: 

copyValueToNew(name:&str,newName:String, copyName:&str,xx: dataBook,typ:&str): name of the file, name of the new data which is going to be added  , name of the data you will be copying, the databook, type if it is _M _A or _D  

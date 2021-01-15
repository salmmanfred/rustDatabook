how to use:  

How to structure a rdb file:  
|test_A|This is an array ,b,c,1ad  
|test_M|This is a string or Meta tag   
|test_D|this is a number tag  
add _A or _M or _D to the name to select the types!  
the _A _M and _D will be ignored when using findValue  
so only use the name you specified when trying to look for it.  
  
How to 'compile' a rdb file  
use rdbData(&str) the string is the name of the file.  
to find the position of the data use findValue  
Disclaimer: due to rust for some reason the values will start on 1 and not 0.  
if you try to get value 0 you will only be left with "" or 0  
  
after that you will be given a dataBook ( from rdbData):  
pub struct dataBook{  
    pub NM:VecString, NM = name meta  
    pub ND:VecString, ND = name digit  
    pub D:Veci64, D = digit  
    pub M:VecString, M = meta  
    pub A:VecVecString, A = array  
    pub AN:VecString AN = array name  
}
you dont need to make your own dataBook as it can be found in rustDatabook::dataBook  
  
How to use find value  
 findValue(&str,&dataBook,&str) -> usize first str is the name of the value and the second str is what type so _A _M _D  

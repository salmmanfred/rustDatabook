mod rdbhandeling;
//mod rdbhandeling::dataBook;




pub fn rdbData(name:&str) -> dataBook{
    return returner(name);
}
pub fn findValue(name: &str, xs: &dataBook,typ: &str) -> usize{
    return findValueB(name, xs, typ)
}
pub fn addData(name:&str,newdata:&str){
    rdbhandeling::addData(name, newdata)
}
pub fn changeData(name:&str,newdata:&str,cname:&str){
    rdbhandeling::changeData(name, newdata,cname)
}
pub fn removeData(name:&str,newdata:&str){
    rdbhandeling::removeData(name, newdata)
}
pub struct dataBook{
    pub NM:Vec<String>,
    pub ND:Vec<String>,
    pub D:Vec<i64>,
    pub M:Vec<String>,
    pub A:Vec<Vec<String>>,
    pub AN:Vec<String>
}

 fn returner(name: &str) -> dataBook{
    
    let  XX:dataBook = dataBook{
        NM : rdbhandeling::getVal_MN(name),
        ND : rdbhandeling::getVal_ND(name),
        D : rdbhandeling::getVal_D(name),
        M : rdbhandeling::getVal_M(name),
        A : rdbhandeling::getVal_A(name),
        AN: rdbhandeling::getVal_AN(name)
        
    
    };
    return XX;
}
fn findValueB(name: &str, xs: &dataBook,typ: &str) -> usize{
    // the if type checks what type and it then goes on to finding the dataname
    if typ == "_D"{
        for x in 0..xs.ND.len(){
            if xs.ND[x] == name.to_string(){
                return x;
            } 
        }
    }else if typ == "_M"{
        for x in 0..xs.NM.len(){
            if xs.NM[x] == name.to_string(){
                return x;
            } 
        }
    }else if typ == "_A"{
        for x in 0..xs.AN.len(){
            if xs.AN[x] == name.to_string(){
                return x;
            } 
        }
    }
    
    return 0;
}
pub fn copyValueToNew(name:&str,newName:String, copyName:&str,xx: dataBook,typ:&str){
    let mut x: String = "".to_string();
    if typ == "_M"{
        
        x = "|".to_owned()+&newName+&"_M|".to_owned()+&xx.M[findValue(copyName, &xx, "_M")];

    }
    if typ == "_D"{
        
        x = "|".to_owned()+&newName+&"_D|".to_owned()+&xx.D[findValue(copyName, &xx, "_D")].to_string();

    }
    if typ == "_A"{
        let mut xf: String = "".to_string();

        for x in &xx.A[findValue(copyName, &xx, "_A")]{
            let xo = x.to_owned()+",";
            xf.push_str(&xo);
        }
        x = "|".to_owned()+&newName+&"_A|".to_owned()+&xf;

    }
    rdbhandeling::addData(name, &x)
}
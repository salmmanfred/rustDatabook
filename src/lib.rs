mod rdbhandeling;
//mod rdbhandeling::dataBook;




pub fn rdbData(name:&str) -> dataBook{
    return returner(name);
}
pub fn findValue(name: &str, xs: &dataBook,typ: &str) -> usize{
    return findValueB(name, xs, typ)
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
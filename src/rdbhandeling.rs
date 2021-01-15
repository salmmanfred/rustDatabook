
use openfile;
use std::io::{stdin,stdout,Write};
use std::io::{prelude::*,BufReader};
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::string;

//use crate::databook;
//use std::iter::collect;

/*pub struct dataBook{
    pub N:Vec<String>,
    pub ND:Vec<String>,
    pub D:Vec<i64>,
    pub M:Vec<String>,
    pub A:Vec<Vec<String>>,
    pub AN:Vec<String>
}*/

pub fn getVal_D(file: &str) -> Vec<i64>{
    let mut vs: Vec<i64> = vec!(0);
    let match_D = Regex::new(r"(\|(\d*\w*)+_D\|)( ?\d+)").unwrap();
    let fss  = openfile::readFileLines(file);
    for x in fss{
        for cap in match_D.captures_iter(&x) {
            vs.push(cap[3].parse::<i64>().unwrap() as i64);
        }
        
    }
    return vs;

}

pub fn getVal_ND(file: &str) -> Vec<String>{
    let mut vecN : Vec<String> = vec!("".to_string());
    let match_D = Regex::new(r"(\|(\d*\w*)+_D\|)( ?\d+)").unwrap();
    let fss  = openfile::readFileLines(file);
    for x in fss{
        for cap in match_D.captures_iter(&x) {
            vecN.push(cap[2].to_string());
        }
        
    }
    return vecN;

}
 pub fn getVal_M(name: &str) -> Vec<String>{
    let mut msv: Vec<String> = vec!("".to_string());
    let match_M = Regex::new(r"(\|(\d*\w*)+_M\|)(( ?\w* *\w*)*)").unwrap();
    

    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push( cap[3].to_string());
            
            
        }
        
    }
    return msv;
    
}
pub fn getVal_MN(name: &str) -> Vec<String>{
    let mut msv: Vec<String> = vec!("".to_string());
    let match_M = Regex::new(r"(\|(\d*\w*)+_M\|)(( ?\w* *\w*)*)").unwrap();


    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push( cap[2].to_string());
            
            
        }
        
    }
    return msv;
    
}

pub fn getVal_A(name: &str) -> Vec<Vec<String>>{
    let mut msv: Vec<String> = vec!();
    let mut msvn: Vec<Vec<String>> = vec!(vec!("".to_string()));

    let match_M = Regex::new(r"(\|(\d*\w*)+_A\|)(( ?,?\w* *\w* *\d* *)*)").unwrap();


    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push( cap[3].to_string());
            println!("As:{}",cap[3].to_string());
            
            
            
        }
        
    }

    for ust in msv{
        let x= ust.split(",");
        let f = x.map(|s| s.to_string()).collect();
   
        //println!("{}",f[0]);
        msvn.push(f);
       
        

    }
    /*for x in 0..msvn.len(){
        println!("{}",msvn[0][x])
    }*/

    return msvn;
    
}

pub fn getVal_AN(name: &str) -> Vec<String>{
    let mut msv: Vec<String> = vec!("".to_string());
    //let mut msvn: Vec<Vec<String>> = vec;

    let match_M = Regex::new(r"(\|(\d*\w*)+_A\|)(( ?\w* *\w* \d* *)*)").unwrap();


    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push( cap[2].to_string());
            
            
        }
        
    }

    

    return msv;
    
}/*
pub fn findValue(name: &str, xs: &dataBook,typ: &str) -> usize{
    if typ == "_D"{
        for x in 0..xs.ND.len(){
            if xs.ND[x] == name.to_string(){
                return x;
            } 
        }
    }else if typ == "_M"{
        for x in 0..xs.N.len(){
            if xs.N[x] == name.to_string(){
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

pub fn returner(name: &str) -> dataBook{
    
    let  XX:dataBook = dataBook{
        N : getVal_MN(name),
        ND : getVal_ND(name),
        D : getVal_D(name),
        M : getVal_M(name),
        A : getVal_A(name),
        AN: getVal_AN(name)
        
    
    };
    return XX;
}*/



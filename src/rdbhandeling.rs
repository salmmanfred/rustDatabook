
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
    //the regex that gets the data
    let match_D = Regex::new(r"(\|(\d*\w*)+_D\|)( ?\d+)").unwrap();
    let fss  = openfile::readFileLines(file);
    // reads the file then parses it
    for x in fss{
        for cap in match_D.captures_iter(&x) {
            vs.push(cap[3].parse::<i64>().unwrap() as i64);
        }
        
    }
    //return the data
    return vs;

}

pub fn getVal_ND(file: &str) -> Vec<String>{
    // starts a new vec
    let mut vecN : Vec<String> = vec!("".to_string());
    // regex for getting data
    let match_D = Regex::new(r"(\|(\d*\w*)+_D\|)( ?\d+)").unwrap();
    let fss  = openfile::readFileLines(file);
    // read data and parse it 
    for x in fss{
        for cap in match_D.captures_iter(&x) {
            vecN.push(cap[2].to_string());
        }
        
    }
    return vecN;

}
// gets metadata
 pub fn getVal_M(name: &str) -> Vec<String>{
    let mut msv: Vec<String> = vec!("".to_string());
    let match_M = Regex::new(r"(\|(\d*\w*)+_M\|)(( ?\w* *\w*)*)").unwrap();
    

    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push(cap[3].to_string());
            
            
        }
        
    }
    return msv;
    
}
// gets the name of meta data
pub fn getVal_MN(name: &str) -> Vec<String>{
    let mut msv: Vec<String> = vec!("".to_string());
    let match_M = Regex::new(r"(\|(\d*\w*)+_M\|)(( ?\w* *\w*)*)").unwrap();


    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push(cap[2].to_string());
            
            
        }
        
    }
    return msv;
    
}
// same as the ones before
pub fn getVal_A(name: &str) -> Vec<Vec<String>>{
    let mut msv: Vec<String> = vec!();
    let mut msvn: Vec<Vec<String>> = vec!(vec!("".to_string()));

    let match_M = Regex::new(r"(\|(\d*\w*)+_A\|)(( ?,?\w* *\w* *\d* *)*)").unwrap();


    let fss  = openfile::readFileLines(name);
    for x in fss{
        
        for cap in match_M.captures_iter(&x) {
            
            msv.push( cap[3].to_string());
            //println!("As:{}",cap[3].to_string());
            
            
            
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
    
}


pub fn addData(name: &str,newdata:&str){
    let mut data = openfile::readFileLines(name);
    data.push(newdata.to_owned());
    let mut datax: String = "".to_string();
    //just adds the data
    for x in data{
        let datap = x+"\n";
        datax += &datap;
    }
    let x = openfile::writeFile(name, &datax);
}
pub fn changeData(name: &str, newdata:&str, cname:&str){
    let mut data = openfile::readFileLines(name);
    data.push(newdata.to_owned());
    let mut datax: String = "".to_string();
    let matcs = Regex::new(r"(\|(\d*\w*)+(_M|_D|_A)\|)").unwrap();
    //reads the data and then changes the data form the dataname
    for x in data{
        for cap in matcs.captures_iter(&x) {
            if &cap[2] == cname{
                let datap = cap[0].to_string()+&newdata.to_string()+"\n";
                datax += &datap;
            }else{
                let datap: String = x.to_string()+&"\n".to_string();
                datax += &datap;
            }
        }
        
    }
    let x = openfile::writeFile(name, &datax);
}
pub fn removeData(name: &str, cname:&str){
    let mut data = openfile::readFileLines(name);
    //data.push(newdata.to_owned());
    let mut datax: String = "".to_string();
    let matcs = Regex::new(r"(\|(\d*\w*)+(_M|_D|_A)\|)").unwrap();
    //parses the data and does not add the data back again after
    for x in data{
        for cap in matcs.captures_iter(&x) {
            if &cap[2] == cname{
               
            }else{
                let datap: String = x.to_string()+&"\n".to_string();
                datax += &datap;
            }
        }
        
    }
    let x = openfile::writeFile(name, &datax);
}
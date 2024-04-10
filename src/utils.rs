use std::fmt::Debug;

use crate::datacell::DataCell;
use crate::handle_error::Err_Handler;
use crate::handle_error::MyResult;

#[derive(Debug)]
pub enum DATA {
    DATE,
    DATETIME,
    TIME,
    NUM,
    BOOL,
    STR,
    ERROR,
    NONE,
}

pub enum AXIS{
    VERTICAL,
    HORIZANTAL
}

#[derive(Debug)]
pub enum KEEP {
    FIRST,
    LAST
}

#[derive(Debug)]
pub enum TIMEDELTA{
    DAYS(i64)
}

pub fn get_timedelta(c: &TIMEDELTA) ->chrono::Duration{
    match c {
        TIMEDELTA::DAYS(x) => chrono::Duration::days(*x)
    }
}

pub fn is_num(c:&DataCell)->bool{
    match c {
        DataCell::F(_) =>true,
        _ => false
    }
}

pub fn is_bool(c:&DataCell)->bool{
    match c {
        DataCell::B(_) =>true,
        _ => false
    }
}

pub fn is_string(c:&DataCell)->bool{
    match c {
        DataCell::S(_) =>true,
        _ => false
    }
}

pub fn is_datetime(c:&DataCell)->bool{
    match c {
        DataCell::DT(_) =>true,
        _ => false
    }
}

pub fn is_date(c:&DataCell)->bool{
    match c {
        DataCell::D(_) =>true,
        _ => false
    }
}

pub fn is_time(c:&DataCell)->bool{
    match c {
        DataCell::T(_) =>true,
        _ => false
    }
}

pub fn is_err(c:&DataCell)->bool{
    match c {
        DataCell::ERR =>true,
        _ => false
    }
}

pub fn is_null(c:&DataCell)->bool{
    match c {
        DataCell::NULL =>true,
        _ => false
    }
}

pub fn get_num(c:&DataCell) ->MyResult<f64>{
    match c {
        &DataCell::F(x) => Ok(x),
        _ => Err(Err_Handler::Type_Error { 
            from: "not num type cannot get as a num".to_string(), 
            reason: format!("{:?} is not num" ,&c)
        })?
    }
}

pub fn get_bool(c:&DataCell) ->MyResult<bool>{
    match c {
        &DataCell::B(x) => Ok(x),
        _ => Err(Err_Handler::Type_Error { 
            from: "not bool type cannot get as a bool".to_string(), 
            reason: format!("{:?} is not bool" ,&c)
        })?
    }
}

pub fn get_string(c:&DataCell) ->MyResult<String>{
    match c {
        DataCell::S(x) => Ok(x.clone()),
        _ => Err(Err_Handler::Type_Error { 
            from: "not string type cannot get as a string".to_string(), 
            reason: format!("{:?} is not string" ,&c)
        })?
    }
}

pub fn get_datetime(c:&DataCell) ->MyResult<chrono::NaiveDateTime>{
    match c {
        &DataCell::DT(x) => Ok(x),
        _ => Err(Err_Handler::Type_Error { 
            from: "not datetime type cannot get as a datetime".to_string(), 
            reason: format!("{:?} is not datetime" ,&c)
        })?
    }
}

pub fn get_date(c:&DataCell) ->MyResult<chrono::NaiveDate>{
    match c {
        &DataCell::D(x) => Ok(x),
        _ => Err(Err_Handler::Type_Error { 
            from: "not date type cannot get as a date".to_string(), 
            reason: format!("{:?} is not date" ,&c)
        })?
    }
}

pub fn get_time(c:&DataCell) ->MyResult<chrono::NaiveTime>{
    match c {
        &DataCell::T(x) => Ok(x),
        _ => Err(Err_Handler::Type_Error { 
            from: "not time type cannot get as a time".to_string(), 
            reason: format!("{:?} is not time" ,&c)
        })?
    }
}

pub fn drop_duplicates<T:Clone+PartialEq>(arr:Vec<T>)->Vec<T>{
    let mut newarr:Vec<T> = vec![];
    for each in arr{
        if !newarr.contains(&each) {
            newarr.push(each.clone());
        }
    }
    newarr
}

#[derive(Debug, Clone)]
pub struct Arg_usize{
    pub data: Vec<usize>
}

impl From<usize> for Arg_usize {
    fn from(c: usize) -> Self {
        Arg_usize { data: vec![c] }
    }
}

impl From<Vec<usize>> for Arg_usize{
    fn from(c: Vec<usize>) -> Self {
        Arg_usize { data: c }
    }
}

impl<const N:usize> From<[usize; N]> for Arg_usize {
    fn from(c: [usize; N]) -> Self {
        Arg_usize { data: c.to_vec() }
    }
}
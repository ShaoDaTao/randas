use xl::ExcelValue;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;

use crate::utils::DATA;
use crate::utils::TIMEDELTA;
use crate::utils::get_bool;
use crate::utils::get_date;
use crate::utils::get_datetime;
use crate::utils::get_num;
use crate::utils::get_string;
use crate::utils::get_time;
use crate::utils::get_timedelta;
use crate::utils::is_bool;
use crate::utils::is_date;
use crate::utils::is_datetime;
use crate::utils::is_num;
use crate::utils::is_string;
use crate::utils::is_time;
use crate::handle_error::Err_Handler;
use crate::handle_error::MyResult;

#[derive(Clone)]
pub enum DataCell{
    F(f64),
    B(bool),
    S(String),
    DT(NaiveDateTime),
    D(NaiveDate),
    T(NaiveTime),
    ERR,
    NULL
}

impl DataCell {
    pub fn str(&self) -> String{
        match self {
            DataCell::B(x) => x.to_string(),
            DataCell::F(x) => x.to_string(),
            DataCell::D(x) => x.to_string(),
            DataCell::DT(x) => x.to_string(),
            DataCell::T(x) => x.to_string(),
            DataCell::S(x) => x.to_string(),
            DataCell::ERR => "".to_string(),
            DataCell::NULL => "".to_string(),
        }
    }

    pub fn fillna<T:Into<DataCell>>(&mut self, m:T){
        *self = match *self {
            DataCell::ERR => m.into(),
            DataCell::NULL => m.into(),
            _ => self.clone()
        }
    }

    pub fn force_astype(&mut self, default: i32){
        
    }

    pub fn astype(&self, t: DATA) -> MyResult<DataCell>{
        match t {
            DATA::NUM => {
                if is_string(&self){
                    let s = get_string(&self)?;
                    let f = s.parse::<f64>().map_err(|e|{
                        Err_Handler::PARSE_ERR { 
                            from: e.to_string(), 
                            reason: format!("parse {} to f64 failed", &s) }
                    })?;
                    Ok(DataCell::F(f))
                }
                else{
                    Err(Err_Handler::PARSE_ERR { 
                        from: "convert string to num failed".to_string(), 
                        reason: format!("only string can be converted to num, found:{:?}", &self) })
                }
            },
            DATA::BOOL => {
                if is_string(&self){
                    let s = get_string(&self)?.to_lowercase();
                    let b = s.parse::<bool>().map_err(|e|{
                        Err_Handler::PARSE_ERR { 
                            from: e.to_string(), 
                            reason: format!("parse {} to bool failed", &s) }
                    })?;
                    Ok(DataCell::B(b))
                }else{
                    Err(Err_Handler::PARSE_ERR { 
                        from: "convert string to bool failed".to_string(), 
                        reason: format!("only string can be converted to bool, found:{:?}", &self) })
                }
            },
            DATA::STR => {
                match self {
                    DataCell::B(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::F(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::S(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::D(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::DT(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::T(x) => Ok(DataCell::S(x.to_string())),
                    DataCell::ERR => Ok(DataCell::S("ERR".to_string())),
                    DataCell::NULL => Ok(DataCell::S("NULL".to_string())),
                }
            },
            DATA::DATE => {
                if is_string(&self){
                    let s = get_string(&self)?;
                    let d = chrono::NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d").map_err(|e|{
                        Err_Handler::PARSE_ERR { 
                            from: e.to_string(), 
                            reason: format!("date format should be %Y-%m-%d, value: {} parse failed", &s) }
                    })?;
                    Ok(DataCell::D(d))
                }else{
                    Err(Err_Handler::PARSE_ERR { 
                        from: "convert to date failed".to_string(), 
                        reason:format!("only string can be converted to date, found:{:?}", &self) })
                }
            },
            DATA::DATETIME => {
                if is_string(&self){
                    let s = get_string(&self)?;
                    let dt = chrono::NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%d %H:%M:%S").map_err(|e|{
                        Err_Handler::PARSE_ERR { 
                            from: e.to_string(), 
                            reason: format!("datetime format should be %Y-%m-%d %H:%M:%s, value: {} parse failed", &s) }
                    })?;
                    Ok(DataCell::DT(dt))
                }else{
                    Err(Err_Handler::PARSE_ERR { 
                        from: "convert to date failed".to_string(), 
                        reason: format!("only string can be converted to datetime, found:{:?}", &self) })
                }
            },
            DATA::TIME => {
                if is_string(&self){
                    let s = get_string(&self)?;
                    let t = chrono::NaiveTime::parse_from_str(s.as_str(), "%H-%M-%S").map_err(|e|{
                        Err_Handler::PARSE_ERR { 
                            from: e.to_string(), 
                            reason: format!("time format should be %H-%M-%S, value:{} parse failed", &s) }
                    })?;
                    Ok(DataCell::T(t))
                }else{
                    Err(Err_Handler::PARSE_ERR { 
                        from: "convert to time failed".to_string(), 
                        reason: format!("only string can be converted to time, found:{:?}", &self) })
                }
            },
            _ => Err(Err_Handler::Not_Matched_Err { 
                from: "wrong type".to_string(), 
                reason: format!("wrong type, found {:?}", &t) })
        }
    }
}

impl std::fmt::Debug for DataCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataCell::B(x) => write!(f, "{:?}", x),
            DataCell::F(x) => write!(f, "{:?}", x),
            DataCell::S(x) => write!(f, "{:?}", x),
            DataCell::DT(x) => write!(f, "{:?}", x),
            DataCell::D(x) => write!(f, "{:?}", x),
            DataCell::T(x) => write!(f, "{:?}", x),
            DataCell::ERR => write!(f, "err"),
            DataCell::NULL => write!(f, "null"),
        }
    }
}

impl std::default::Default for DataCell {
    fn default() -> Self {
        DataCell::NULL
    }
}

pub trait Build{
    fn new(self)->DataCell;
}

impl Build for usize{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for u32{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for u64{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for f32{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for i32{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for f64{
    fn new(self)->DataCell{
        DataCell::F(self)
    }
}

impl Build for i64{
    fn new(self)->DataCell{
        DataCell::F(self as f64)
    }
}

impl Build for bool{
    fn new(self)->DataCell{
        DataCell::B(self)
    }
}

impl Build for String{
    fn new(self)->DataCell{
        DataCell::S(self)
    }
}

impl Build for &str{
    fn new(self)->DataCell{
        DataCell::S(String::from(self))
    }
}

impl std::fmt::Display for DataCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl From<f64> for DataCell {
    fn from(c: f64) -> Self {
        DataCell::F(c)
    }
}

impl From<i64> for DataCell {
    fn from(c: i64) -> Self {
        DataCell::F(c as f64)
    }
}

impl From<f32> for DataCell {
    fn from(c: f32) -> Self {
        DataCell::F(c as f64)
    }
}

impl From<i32> for DataCell {
    fn from(c: i32) -> Self {
        DataCell::F(c as f64)
    }
}

impl From<bool> for DataCell {
    fn from(c: bool) -> Self {
        DataCell::B(c)
    }
}

impl From<String> for DataCell {
    fn from(c: String) -> Self {
        DataCell::S(c)
    }
}

impl From<&str> for DataCell {
    fn from(c: &str) -> Self {
        DataCell::S(c.to_string())
    }
}

impl From<&ExcelValue<'_>> for DataCell  {
    fn from(c: &ExcelValue) -> Self {
        let a = match c {
            ExcelValue::Bool(x) => DataCell::B(*x),
            ExcelValue::Date(x) => DataCell::D(*x),
            ExcelValue::DateTime(x) => DataCell::DT(*x),
            ExcelValue::Error(_) => DataCell::ERR,
            ExcelValue::None => DataCell::NULL,
            ExcelValue::Number(x) => DataCell::F(*x),
            ExcelValue::String(x) => DataCell::S(x.to_string()),
            ExcelValue::Time(x) => DataCell::T(*x),
        };
        a.clone()
    }
}


impl Add for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            DataCell::F(x) => {
                if is_num(&rhs){
                    let val2 = get_num(&rhs).unwrap();
                    Ok(DataCell::F(x + val2))
                }else{ 
                    Err(Err_Handler::Different_Type_Cannot_Add { 
                        from: "wrong type cannot be added".to_string(), 
                        reason: format!("{:?} not num, cannot be added", &rhs) })
                 }
            },
            DataCell::S(x) => {
                if is_string(&rhs){
                    let val2 = get_string(&rhs).unwrap();
                    Ok(DataCell::S(format!("{}{}",x,val2)))
                }else{ Err(Err_Handler::Different_Type_Cannot_Add { 
                    from: "wrong type cannot be added".to_string(), 
                    reason: format!("{:?} not string, cannot be added", &rhs) })
                }
            },
            DataCell::B(x) =>{
                if is_bool(&rhs){
                    let val2 = get_bool(&rhs).unwrap();
                    if x & val2 {
                        Ok(DataCell::B(true))
                    }else{
                        Ok(DataCell::B(false))
                    }
                }else{
                    Err(Err_Handler::Different_Type_Cannot_Add { 
                        from: "wrong type cannot be added".to_string(), 
                        reason: format!("{:?} not bool, cannot be added", &rhs) })
                }
            },
            _ =>Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added", &rhs) })
        }
    }
}

impl Add<TIMEDELTA> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: TIMEDELTA) -> Self::Output {
        match self {
            DataCell::D(x) => Ok(DataCell::D(x + get_timedelta(&rhs))),
            DataCell::DT(x) => Ok(DataCell::DT(x + get_timedelta(&rhs))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for TIMEDELTA {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::D(x) => Ok(DataCell::D(x + get_timedelta(&self))),
            DataCell::DT(x) => Ok(DataCell::DT(x + get_timedelta(&self))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for i32 {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::F(x) =>Ok(DataCell::F(x + self as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<i32> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            DataCell::F(x) => Ok(DataCell::F(x+rhs as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<f32> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: f32) -> Self::Output {
        match self {
            DataCell::F(x) => Ok(DataCell::F(x + rhs as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for f32 {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::F(x) => Ok(DataCell::F(x + self as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}


impl Add<i64> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: i64) -> Self::Output {
        match self {
            DataCell::F(x) => Ok(DataCell::F(x + rhs as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for i64 {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::F(x) => Ok(DataCell::F(x + self as f64)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<f64> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: f64) -> Self::Output {
        match self {
            DataCell::F(x) => Ok(DataCell::F(x + rhs)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for f64 {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::F(x) => Ok(DataCell::F(x + self)),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<String> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: String) -> Self::Output {
        match self {
            DataCell::S(x) => Ok(DataCell::S(format!("{}{}", x, rhs))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for String {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::S(x) => Ok(DataCell::S(format!("{}{}", self, x))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<&str> for DataCell {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: &str) -> Self::Output {
        match self {
            DataCell::S(x) => Ok(DataCell::S(format!("{}{}", x, rhs))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

impl Add<DataCell> for &str {
    type Output = MyResult<DataCell>;
    fn add(self, rhs: DataCell) -> Self::Output {
        match rhs {
            DataCell::S(x) => Ok(DataCell::S(format!("{}{}", self, x))),
            _ => Err(Err_Handler::Different_Type_Cannot_Add { 
                from: "wrong type cannot be added".to_string(), 
                reason: format!("{:?} cannot be added here", &rhs) }),
        }
    }
}

//减
// impl Sub for DataCell {
//     type Output = DataCell;
//     fn sub(self, rhs: Self) -> Self::Output {
//         match self {
//         }
//     }
// }

//比较大小
impl std::cmp::Eq for DataCell {
    // fn eq(&self, other: &Self) -> bool {
    //     match self {
    //         DataCell::B(x) => {
    //             if is_bool(other){
    //                 if *x == get_bool(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         DataCell::F(x) =>{
    //             if is_num(other){
    //                 if *x == get_num(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         DataCell::D(x) => {
    //             if is_date(other){
    //                 if *x == get_date(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         DataCell::DT(x) => {
    //             if is_datetime(other){
    //                 if *x == get_datetime(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         DataCell::T(x) => {
    //             if is_time(other){
    //                 if *x == get_time(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         DataCell::S(x) => {
    //             if is_string(other){
    //                 if *x == get_string(other).unwrap(){
    //                     true
    //                 }else{
    //                     false
    //                 }
    //             }else{
    //                 false
    //             }
    //         },
    //         _ => false
    //     }
    // }
}



impl std::cmp::PartialEq for DataCell {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DataCell::B(x) => {
                if is_bool(other){
                    if *x == get_bool(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            DataCell::F(x) =>{
                if is_num(other){
                    if *x == get_num(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            DataCell::D(x) => {
                if is_date(other){
                    if *x == get_date(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            DataCell::DT(x) => {
                if is_datetime(other){
                    if *x == get_datetime(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            DataCell::T(x) => {
                if is_time(other){
                    if *x == get_time(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            DataCell::S(x) => {
                if is_string(other){
                    if *x == get_string(other).unwrap(){
                        true
                    }else{
                        false
                    }
                }else{
                    false
                }
            },
            _ => false
        }
    }
}

impl std::cmp::Ord for DataCell {
    fn cmp(&self, other: &Self) -> Ordering {
        if is_num(&self) & is_num(&other){
            if get_num(self).unwrap() > get_num(other).unwrap() {
                Ordering::Greater
            }else if  get_num(self).unwrap() == get_num(other).unwrap(){
               Ordering::Equal
            }else{
                Ordering::Less
            }
        }else if is_date(&self) & is_date(&other){
            if get_date(self).unwrap() > get_date(other).unwrap(){
                Ordering::Greater
            }else if get_date(self).unwrap() == get_date(other).unwrap(){
                Ordering::Equal
            }else{
                Ordering::Less
            }
        }else if is_datetime(&self) & is_datetime(&other){
            if get_datetime(self).unwrap() > get_datetime(other).unwrap(){
                Ordering::Greater
            }else if get_datetime(self).unwrap() == get_datetime(other).unwrap(){
                Ordering::Equal
            }else{
                Ordering::Less
            }
        }else if is_time(&self) & is_time(&other){
            if get_time(self).unwrap() > get_time(other).unwrap(){
                Ordering::Greater
            }else if get_time(self).unwrap() == get_time(other).unwrap(){
                Ordering::Equal
            }else{
                Ordering::Less
            }
        }else if is_bool(&self) & is_bool(&other){
            if get_bool(self).unwrap() == get_bool(other).unwrap() {
                Ordering::Equal
            }else{
                // panic!("bool cannot be compared as greater or less")
                panic!("bool cannot be compared as  greater or less")
            }
        }else{
            panic!("you are trying to sort the datacell but only num, date, datetime, time can be sorted, and bool can be equal. others are invalid to compare.")
        }
    }
}

impl std::cmp::PartialOrd for DataCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if is_num(&self) & is_num(&other){
            if get_num(self).unwrap() > get_num(other).unwrap() {
                Some(Ordering::Greater)
            }else if  get_num(self).unwrap() == get_num(other).unwrap(){
               Some(Ordering::Equal)
            }else{
                Some(Ordering::Less)
            }
        }else if is_date(&self) & is_date(&other){
            if get_date(self).unwrap() > get_date(other).unwrap(){
                Some(Ordering::Greater)
            }else if get_date(self).unwrap() == get_date(other).unwrap(){
                Some(Ordering::Equal)
            }else{
                Some(Ordering::Less)
            }
        }else if is_datetime(&self) & is_datetime(&other){
            if get_datetime(self).unwrap() > get_datetime(other).unwrap(){
                Some(Ordering::Greater)
            }else if get_datetime(self).unwrap() == get_datetime(other).unwrap(){
                Some(Ordering::Equal)
            }else{
                Some(Ordering::Less)
            }
        }else if is_time(&self) & is_time(&other){
            if get_time(self).unwrap() > get_time(other).unwrap(){
                Some(Ordering::Greater)
            }else if get_time(self).unwrap() == get_time(other).unwrap(){
                Some(Ordering::Equal)
            }else{
                Some(Ordering::Less)
            }
        }else if is_bool(&self) & is_bool(&other){
            if get_bool(self).unwrap() == get_bool(other).unwrap() {
                Some(Ordering::Equal)
            }else{
                // panic!("bool cannot be compared as greater or less")
                None
            }
        }else{
            // panic!("the type provided cannot be compared.")
            None
        }
    }

    fn le(&self, other: &Self) -> bool {
        let a = self.partial_cmp(other);
        match a {
            Some(Ordering::Greater) => false,
            Some(Ordering::Less) => true,
            Some(Ordering::Equal) => true,
            _ => panic!("invalid comparison")
            // _ => None
        }
    }

    fn gt(&self, other: &Self) -> bool {
        let a = self.partial_cmp(other);
        match a {
            Some(Ordering::Greater) => true,
            Some(Ordering::Less) => false,
            // Some(Ordering::Equal) => false,
            _ => panic!("invalid comparison")
        }
    }

    fn ge(&self, other: &Self) -> bool {
        let a = self.partial_cmp(other);
        match a {
            Some(Ordering::Greater) => true,
            Some(Ordering::Less) => false,
            Some(Ordering::Equal) => true,
            _ => panic!("invalid comparison")
        }
    }

    fn lt(&self, other: &Self) -> bool {
        let a = self.partial_cmp(other);
        match a {
            Some(Ordering::Greater) => false,
            Some(Ordering::Less) => true,
            _ => panic!("invalid comparison")
        }
    }
}
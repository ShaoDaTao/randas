
use std::ops::{Index, Add};

use crate::datacell::*;
#[derive(Clone)]
pub struct DataSeries {
    pub title:String,
    pub vals: Vec<DataCell>
}

impl DataSeries {
    fn astype(&mut self) {
        
    }

    fn force_astype(&mut self, default: i32){
        
    }

    // fn sort(&mut self){
    //     *self.vals.sort()
    // }

    fn fillna(&mut self){
        
    }
}

impl IntoIterator for DataSeries {
    type IntoIter = std::vec::IntoIter<DataCell>;
    type Item = DataCell;
    fn into_iter(self) -> Self::IntoIter {
        self.vals.into_iter()
    }
}

impl std::fmt::Debug for DataSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}\n", &self.title, &self.vals)
    }
}

// impl Add for DataSeries {
//     type Output = DataSeries;
//     fn add(self, rhs: Self) -> Self::Output {
        
//     }
// }

// i64
// impl Add<DataSeries> for i64 {
//     type Output = DataSeries;
//     fn add(self, rhs: DataSeries) -> Self::Output {
        
//     }
// }

// impl Add<i64> for DataSeries {
    
// }

// // f64
// impl Add<DataSeries> for f64 {
    
// }

// impl Add<f64> for DataSeries {
    
// }

// impl Add<DataSeries> for String {
    
// }

// impl Add<String> for DataSeries {
    
// }
// pub mod datacell;
// pub mod dataframe;
// pub mod dataseries;
// pub mod utils;
// pub mod macros;
// pub mod handle_error;
// pub mod groupby;

// use crate::datacell::Build;
// use datacell::DataCell;
// use xl::Workbook;
// use xl::Worksheet;
// use std::collections::HashMap;
// use crate::dataframe::DataFrame as pd;
// use crate::dataseries::DataSeries;
// use crate::dataframe::DataFrame;
// use chrono;
// use crate::groupby::split_to_group;

fn main() {
    println!("ok");
    // let mut a = DataFrame::read_excel("abc.xlsx", 0, "Sheet1").unwrap();
    // println!("{:?}", &a);
    // let news = series!("A".to_string()=>[1,1.1,"abc".to_string()]);
    // a["location"] = news;
    // let b = a.sort(true);
    // println!("{:?}", &b);
    
    // println!("{:?}", a["qty"]);
    // let c = DataSeries{
    //     title: "test".to_string(),
    //     vals: vec![DataCell::F(2.2), DataCell::F(2.2), DataCell::F(2.2)]
    // };
    // a[2] = c;
    // println!("{:?}", a[1]);
    // println!("{:?}", c);
    
    // let mut d = vec![1,2,3,4,3];
    // d[0] = 100;
    // println!("{:?}", d);

    // println!("{:?}", a);
    // let a = "12390".to_string();
    // let b = a.parse::<f64>().unwrap();
    // println!("{:?}", b);

    // let a = series!("A".to_string()=>[1,1.1,2,"abc".to_string()]);
    // println!("{:?}", a);

    // let mut df = pd::read_excel(r"abc.xlsx", 0, "Sheet1");
    // println!("{:?}", df);
    // df.to_excel(r"abcdefg.xlsx", "qwe");

    // let a = DataCell::F(1.3);
    // let b = DataCell::F(1.2);
    // let e = DataCell::F(1.25);
    // let c = DataCell::B(true);
    // let c1 = DataCell::B(true);
    // let d = DataCell::B(false);
    // let mut aaa = DataCell::ERR;
    // let mut aaa = vec![a,b,e];
    // aaa.sort();
    // aaa.reverse();
    
// //对二维数组进行分组. 分组参数可以若干个.
// let mut xx = vec![vec![vec![DataCell::F(2.0),DataCell::F(4.0),DataCell::F(11.0)], 
//                                             vec![DataCell::F(1.0),DataCell::F(8.0),DataCell::F(3.0)], 
//                                             vec![DataCell::F(2.0),DataCell::F(3.0),DataCell::F(5.0)], 
//                                             vec![DataCell::F(3.0),DataCell::F(3.0),DataCell::F(1.0)], 
//                                             vec![DataCell::F(2.0),DataCell::F(3.0),DataCell::F(1.0)], 
//                                             vec![DataCell::F(1.0),DataCell::F(2.0),DataCell::F(10.0)],
//                                             vec![DataCell::F(1.0),DataCell::F(8.0),DataCell::F(11.0)],
//                                             vec![DataCell::F(1.0),DataCell::F(2.0),DataCell::F(5.0)]]];
// //参数0, 1,表示针对数组arr, 优先按每个数组第一个元素排序, 再按第2个元素排序, 最后分成若干组.
// split_to_group(&mut xx, vec![0, 1]);        

// println!("{:?}", xx);


}

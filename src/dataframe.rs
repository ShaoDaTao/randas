use crate::datacell::*;
use crate::dataseries::DataSeries;
use crate::utils::get_string;
use std::fmt::format;
// use &std::error::Error;
use std::path::Path;
use simple_excel_writer::Cell;
use xl::Workbook;
use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;
use crate::utils::KEEP;
use simple_excel_writer::Workbook as EXCELWriter;
use simple_excel_writer::Row as WriteRow;
use xl::ExcelValue;

use crate::utils::Arg_usize;
use simple_excel_writer::SheetWriter;

use crate::handle_error::Err_Handler;
use crate::handle_error::MyResult;

#[derive(Clone)]
pub struct DataFrame {
    pub columns: Vec<DataSeries>
}

impl std::fmt::Debug for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = &self.to_row_arr();
        write!(f, "DataFrame:\n")?;
        write!(f, "---------------BEGIN--------------\n")?;
        for row in rows{
            write!(f, "{:?}\n", row)?;
        }
        write!(f, "----------------END---------------")
    }
}

impl DataFrame {
    fn from_col_arr(df_vec: Vec<Vec<DataCell>>)->MyResult<Self>{
        let colnum = df_vec.len();
        let rownum = df_vec[0].len();
        if colnum==0 || rownum==0{
            Err_Handler::Null_Result { 
                from: "convert vec to dataframe".to_string(), 
                reason: "blank vec cannot be converted to dataframe".to_string()};
        }
        let mut df = DataFrame{ columns: vec![]};
        for col in df_vec{
            let title = get_string(&col[0]).unwrap();
            let mut v :Vec<DataCell>= vec![];
            let mut i = 0;
            for c in col{
                if i>0{
                    v.push(c);
                }
                i += 1;
                
            }
            let tmp_dataseries = DataSeries {
                title: title,
                vals: v
            };
            df.columns.push(tmp_dataseries);
        }
        Ok(df)
    }

    fn from_row_arr(df_vec: Vec<Vec<DataCell>>)->Self{
        let colcount = df_vec[0].len();
        let rowcount = df_vec.len();
        let mut r = DataFrame{columns:vec![]};
        for i in 0..colcount{
            let mut d:DataSeries = DataSeries { title: df_vec[0][i].str(), vals: vec![] };
            r.columns.push(d);
        }
        for x in 1..rowcount{
            for y in 0..colcount{
                r.columns[y].vals.push(df_vec[x][y].clone());
            }
        }
        r
    }

    pub fn read_excel(path:&str, head:u32, sheetname: &str)->MyResult<DataFrame>{
        let mut wb = Workbook::open(path).map_err(|e|{
            Err_Handler::File_Not_Found { from: e.to_string(), reason: format!("File not found: {:?}", path) }
        })?;
        let sheets = &wb.sheets();
        let sheet = sheets.get(sheetname).ok_or_else(||{
            Err_Handler::Sheet_Not_Found { from: "sheet not found".to_string(), reason: format!("sheet: {:?} not found", sheetname.to_string()) }
        })?;
        let mut df_vec:Vec<Vec<DataCell>> = vec![];
        for row in sheet.rows(&mut wb) {
            for _ in row.0.iter(){
                df_vec.push(vec![]);
            }
            break;
        }
        let mut i = 0;
        for row in sheet.rows(&mut wb) {
            i = 0;
            for cell in row.0.iter(){
                let tmpval = &cell.value;
                if i==0 {
                    let tmpv1:DataCell = tmpval.into();
                    let tmpcell:DataCell = match tmpv1 {
                        DataCell::F(x) => DataCell::S(x.to_string()),
                        DataCell::B(x) => DataCell::S(x.to_string()),
                        DataCell::S(x) => DataCell::S(x),
                        DataCell::DT(x) => DataCell::S(x.to_string()),
                        DataCell::D(x) => DataCell::S(x.to_string()),
                        DataCell::T(x) => DataCell::S(x.to_string()),
                        DataCell::ERR => Err(Err_Handler::Title_Error { 
                            from: "Err cell cannot be title".to_string(), 
                            reason: format!("Err cell value cannot be title") })?,
                        DataCell::NULL => Err(Err_Handler::Title_Error { 
                            from: "Null cell cannot be title".to_string(), 
                            reason: format!("Null cell value cannot be title") })?,
                    };
                    df_vec[i].push(tmpcell);
                }else{
                    let tmpcell: DataCell = tmpval.into();
                    df_vec[i].push(tmpcell);
                }
                i += 1;
            }
        }
        let df = Self::from_col_arr(df_vec).unwrap();
        Ok(df)
    }

    pub fn get_titles(&self) ->Vec<String>{
        let mut titles :Vec<String>= vec![];
        for col in &self.columns{
            titles.push(col.title.clone());
        }
        titles
    }

    pub fn get_colcount(&self)->usize {
        self.columns.len()
    }

    //always keep the first. therefore if you want to keep the last, firstly you should sort it to front.
    // pub fn drop_duplicates(&self, by:&str, keep:KEEP){
    //     &self.to_row_arr();
    // }

    pub fn sort<T:Into<Arg_usize>>(&mut self, by: T, ascending: bool)->Self{
        let args = by.into();
        let args0 :Vec<usize>= args.data;
        let mut a = self.to_row_arr();
        let mut b = a.drain(1..).collect::<Vec<Vec<DataCell>>>();
        if ascending {
            b.sort_by(|a, b|{
                let a1 = args0.iter().map(|k|a[k]).collect::<Vec<DataCell>>();
                let mut x = a1.iter().map(|_x|format!("{}|", _x.str())).collect::<Vec<String>>().concat();
                x.pop();
                let mut y = b.iter().map(|_y|format!("{}|", _y.str())).collect::<Vec<String>>().concat();
                y.pop();
                x.partial_cmp(&y).unwrap()
            });
        }else{
            b.sort_by(|a, b|{
                let mut x = a.iter().map(|_x|format!("{}|", _x.str())).collect::<Vec<String>>().concat();
                x.pop();
                let mut y = b.iter().map(|_y|format!("{}|", _y.str())).collect::<Vec<String>>().concat();
                y.pop();
                y.partial_cmp(&x).unwrap()
            });
        }
        a.append(&mut b);
        Self::from_row_arr(a)
    }

    // pub fn sort_by(&self){

    // }

    pub fn get_rowcount(&self)->usize{
        *&self.columns[0].vals.len()
    }

    pub fn to_col_dict(&self)->HashMap<String, &DataSeries>{
        let titles = &self.get_titles();
        let mut r:HashMap<String, &DataSeries> = HashMap::new();
        for t in titles{
            r.insert(t.clone(), &self[t]);
        }
        r
    }

    pub fn add_column(&mut self, s:DataSeries) ->MyResult<&mut Self>{
        let t = &self.get_rowcount();
        let t1 = &s.vals.len();
        if t == t1{
            self.columns.push(s);
        }else{
            Err(Err_Handler::DataSeries_Length_Should_Be_Same { 
                from: "new dataseries length should be same as the target dataframe".to_string(), 
                reason: format!("target length: {:?}, new dataseries length: {:?}", &t, &t1) }
            )?
        }
        Ok(self)
    }

    pub fn to_row_arr(&self) ->Vec<Vec<DataCell>>{
        let rownum = self.get_rowcount();
        let colnum = self.get_colcount();
        let mut result_arr :Vec<Vec<DataCell>>= vec![];
        for r in 0..rownum+1{
            let mut row_vec :Vec<DataCell>= vec![];
            if r==0{
                for c in 0..colnum{
                    let t = self.columns.get(c).unwrap().clone().title;
                    row_vec.push(DataCell::S(t));
                }
            }else{
                for c in 0..colnum{
                    row_vec.push(self.columns.get(c).unwrap().vals[r-1].clone());
                }
            }
            result_arr.push(row_vec)
        }
        result_arr
    }

    pub fn to_cell_dict(&self)->Vec<HashMap<String, DataCell>>{
        let col_dict = &self.to_col_dict();
        let height = self.get_rowcount() as u32;
        let mut r:Vec<HashMap<String, DataCell>> = vec![];
        for i in 0..height{
            let mut rowdict:HashMap<String, DataCell> = HashMap::new();
            for (k, v) in col_dict{
                rowdict.insert(k.clone(), v.vals[i as usize].clone());
            }
            r.push(rowdict);
        }
        r
    }

    pub fn to_excel(&self, filename:&str, sheetname:&str)->MyResult<()>{
        let mut wb = EXCELWriter::create(filename);
        let mut sheet = wb.create_sheet(sheetname);
        let rows = &self.to_row_arr();
        wb.write_sheet(&mut sheet, |sheet_writer|{
            let sw = sheet_writer;
            for row in rows{
                let mut data_row = WriteRow::new();
                for cell in row{
                    match cell.clone() {
                        DataCell::F(x)=>data_row.add_cell(x),
                        DataCell::B(x)=>data_row.add_cell(x),
                        DataCell::S(x)=>data_row.add_cell(x),
                        DataCell::DT(x)=>data_row.add_cell(x.to_string()),
                        DataCell::D(x)=>data_row.add_cell(x.to_string()),
                        DataCell::T(x)=>data_row.add_cell(x.to_string()),
                        DataCell::ERR=>data_row.add_cell(()),
                        DataCell::NULL=>data_row.add_cell(()),
                    }
                }
                sw.append_row(data_row)?;
            }
            let mut endrow = WriteRow::new();
            endrow.add_cell(());
            sw.append_row(endrow)
        }).map_err(|e|{
            Err_Handler::Write_To_File_Failed { from: e.to_string(), reason: format!("write to file failed: {}", &filename) }
        })?;
        wb.close().map_err(|e|{
            Err_Handler::File_Close_Failed { from: e.to_string(), reason: format!("file close failed: {:?}", &filename) }
        })?;
        Ok(())
    }

}

impl IntoIterator for DataFrame {
    type Item = Vec<DataCell>;
    type IntoIter = std::vec::IntoIter<Vec<DataCell>>;
    fn into_iter(self) -> Self::IntoIter {
        let mut tmp_out:Vec<Vec<DataCell>> = vec![];
        for each in self.columns{
            let mut tmp_inner: Vec<DataCell> = vec![];
            for eachcell in each.vals{
                tmp_inner.push(eachcell);
            }
            tmp_out.push(tmp_inner);
        }
        tmp_out.into_iter()
    }
}

impl<'a> Index<&String> for DataFrame {
    // it's strongly recommanded that, before get value by a index, firstly compare the dataframe length and input number: "index", to insure the index is valid.
    // otherwise, a panic would happen.
    type Output = DataSeries;
    fn index(&self, index:&String) -> &Self::Output {
        // let index_string = index.to_string();
        let mut i = 0;
        for each in &self.columns{
            if each.title == *index{
                 break;
            }
            i += 1;
        }
        &self.columns.get(i).expect("index out of range.")
    }
}


impl<'a> Index<&'a str> for DataFrame {
    // it's strongly recommanded that, before get value by a index, firstly compare the dataframe length and input number: "index", to insure the index is valid.
    // otherwise, a panic would happen.
    type Output = DataSeries;
    fn index(&self, index: &'a str) -> &Self::Output {
        let index_string = index.to_string();
        let mut i = 0;
        for each in &self.columns{
            if each.title == index_string{
                 break;
            }
            i += 1;
        }
        &self.columns.get(i).expect("index out of range.")
    }
}

impl IndexMut<&str> for DataFrame {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let index_string = index.to_string();
        let mut i = 0;
        for each in &self.columns{
            if each.title == index_string{
                break;
            }
            i += 1;
        }
        self.columns.get_mut(i).unwrap()
    }
}

impl Index<usize> for DataFrame {
    type Output = DataSeries;
    fn index(&self, index: usize) -> &Self::Output {
        self.columns.get(index).unwrap()
    }
}

impl IndexMut<usize> for DataFrame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.columns.get_mut(index).unwrap()
    }
}
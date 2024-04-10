use std::{fmt::Debug, usize};

#[derive(Debug)]
struct MyData<T:Clone>{
    data: Vec<Vec<T>>
}

impl<T:Clone+Debug> MyData<T> {
    fn group_by<F>(&self, pred: F)->MyGroupBy<T, F>
    where F: FnMut(&Vec<T>, &Vec<T>) ->bool
    {
        MyGroupBy { data: self.data.clone(), predicate: pred }
    }
}

struct MyGroupBy<T:Debug, P>{
    data: Vec<Vec<T>>,
    predicate: P
}

impl<T:Clone+Debug, P> Iterator for MyGroupBy<T, P> 
where
    P: FnMut(&Vec<T>, &Vec<T>) -> bool
{
    type Item = Vec<Vec<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty(){
            None
        }else{
            let mut len = 1;
            let mut iter = self.data.windows(2);
            while let Some([l,r]) = iter.next(){
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let tmp = self.data.clone();
            let (head, tail) = tmp.split_at(len);
            self.data = tail.to_vec();
            Some(head.to_vec())
        }
    }
}

impl<T:Debug, P> std::fmt::Debug for MyGroupBy<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupBy").field("vec", &self.data).finish()
    }
}


pub fn split_to_group<T, P: PartialOrd+Clone+Debug>(arr: &mut T, arg:Vec<usize>) 
where T:AsMut<Vec<Vec<Vec<P>>>>
{
    if arg.len() > 0{
        let (k, rem) = arg.split_first().unwrap();
        let mut r: Vec<Vec<Vec<P>>> = vec![];
        for each in arr.as_mut().iter(){
            let mut tmp = each.clone();
            tmp.sort_by(|a, b|a[k.clone()].partial_cmp(&b[k.clone()]).unwrap());
            let mydata = MyData{ data:tmp };
            let tmp_g = mydata.group_by(|a, b|a[k.clone()]==b[k.clone()]).collect::<Vec<Vec<Vec<P>>>>();
            r.append(&mut tmp_g.to_vec().into_iter().collect::<Vec<Vec<Vec<P>>>>());
        }
        arr.as_mut().clear();
        arr.as_mut().append(&mut r);
        split_to_group(&mut arr.as_mut(), rem.to_vec());
    }
}


// //对二维数组进行分组. 分组参数可以若干个.
// let mut xx = vec![vec![vec![2,4,11], vec![1,8,3], vec![2,3,5], 
//                                             vec![3,3,1], vec![2,3,1], vec![1,2,10],
//                                             vec![1,8,11],vec![1,2,5]]];
// //参数0, 1,表示针对数组arr, 优先按每个数组第一个元素排序, 再按第2个元素排序, 最后分成若干组.
// split_to_group(&mut xx, vec![0, 1]);        

// println!("{:?}", xx);





#[macro_export]
macro_rules! df {
    () => {
        DataFrame{
            columns: vec![series!()]
        }
    };
    (   
        $($t:expr => [$($c:expr), *]), *
                                    ) => {
                                        {
        let mut m = DataFrame{columns:vec![
            $(DataSeries{
                title:$t.to_string(),
                vals:vec![$(Build::new($c)),*]
            }), *
        ]};
            let mut vlen = 0;
            for seriesunit in m.columns.iter(){
                vlen = std::cmp::max(vlen, seriesunit.vals.len());
            }
            for seriesunit in m.columns.iter_mut(){
                if vlen>seriesunit.vals.len(){
                    seriesunit.vals.resize_with(vlen, Default::default);
                }
            }
            m
        }

    }
}


#[macro_export]
macro_rules! series {
    () => {
        DataSeries{
            title: "NA",
            data: vec![]
        }
    };
    ($t:expr => [$($c:expr), *]) => {
        DataSeries{
            title:$t,
            vals:vec![$(Build::new($c)),*]
        }
    };
}
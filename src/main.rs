extern crate classif;
use intervaltree::IntervalTree;

fn main() {
    let range : i64 = 50;
    // this below is the kind of "unwanted" situation
    // which we later need to accound for
    let _tree_with_overlap: IntervalTree<i64, &str> = [
            (1000..1300, "S1"),
            (900..1200, "S2"),
            (1000..1100, "S3"),
            (1250..1500, "S4"),
            (1400..1600, "S5"),
            (1500..1600, "S6"),
            (1500..1700, "S7"),
            (5000..5500, "S8"),
        ].iter().cloned().collect();
    //dbg!(&tree_with_overlap);
    //  900--------------------1200
    //    1000-----------1100
    //    1000-----------------------1300
    //                            1250--------------------1500
    //                                                 1400---------1600
    //                                                    1500------1600
    //                                                    1500--------------1700
    //                                                                                            5000----5500

    // this is the simplified siutation where the element 1250..1500 is removed
    // and therefore we split the 2 clusters easily --> element S4
    let tree_no_overlap: IntervalTree<i64, &str> = [
            (1000..1300, "S1"),
            (900..1200, "S2"),
            (1000..1100, "S3"),
            (1400..1600, "S5"),
            (1500..1600, "S6"),
            (1500..1700, "S7"),
            (5000..5500, "S8"),
        ].iter().cloned().collect();
    //dbg!(&tree_no_overlap);

    // 1st step: get ranges within which we will
    // get all elements from 1 cluster
    let mut break_points1 : Vec<std::ops::Range<i64>> = Vec::new();
    // initialize empty
    let mut prev_end   : Option<i64> = None;
    let mut prev_start : Option<i64 > = None;
    // iterate in sorted (start) order 
    // and define clusters
    for element in tree_no_overlap.iter_sorted(){
        dbg!(&element);
        // GET FIRST ELEMENT
        if prev_start.is_none(){
            prev_start = Some(element.range.start);
        }
        if prev_end.is_none(){
            prev_end = Some(element.range.end);
        }
        if element.range.start - prev_end.unwrap() > range {
            
            let new_range = std::ops::Range {
                start: prev_start.unwrap(),
                end: prev_end.unwrap(),
            };
            eprintln!("cluster start {} cluster end {}", new_range.start,new_range.end);
            break_points1.push(new_range);
            //prev_start = None;
            prev_start = Some(element.range.start);
            prev_end   = Some(element.range.end.clone());
        }else
        // it can happen that the range is still within cluster
        // but end is earlier than existing one due to fact that
        // it iterates based on start, not end
        if element.range.end > prev_end.unwrap() {
            prev_end = Some(element.range.end.clone());
        }
        println!("prev_Start {}, prev_End {}", prev_start.unwrap(),prev_end.unwrap());
    }
    // wrap up last element
    let new_range = std::ops::Range {
        start: prev_start.unwrap(),
        end: prev_end.unwrap(),
    };
    eprintln!("cluster start {} cluster end {}", new_range.start,new_range.end);
    break_points1.push(new_range);
    dbg!(&break_points1);

    // now we can use the generated cluster ranges to get the elements
    // of ranges which match each cluster

    let mut i = 0;
    for cluster in break_points1{
        i += 1;
        eprintln!("Analyzing cluster:{}",i);
        let result = tree_no_overlap.query(cluster);
        dbg!(result);
    };

    /*
    let values : Vec<i64> = [1000,1002,1005,5000,5002,9500,9700,10000,15000,15500,16000,50000].to_vec();
    // so best is if we take now our max distance for clustering and we check which ones
    // are beyond the max distance
    let mut break_points : Vec<i64> = Vec::new();
    let mut prev :i64 = 0;
    for  point in &values  {
        if point -prev > range {
            break_points.push(*point);
        }
        prev = *point;

    }
    dbg!(values);
    dbg!(break_points);
    
    let min = values[0];
    let max = values.last().unwrap();
    let breaks = classif::get_head_tail_breaks(&values).len()-1;
    dbg!(classif::get_head_tail_breaks(&values));
    println!("min {} max {} breaks {}", min,max,breaks as u32);
    println!("values: {:?}",&values);
    dbg!(classif::get_jenks_breaks(&values,breaks as u32));
    dbg!(classif::get_arithmetic_breaks(&values,breaks as u32));
    println!("Hello, world!");
    */
}

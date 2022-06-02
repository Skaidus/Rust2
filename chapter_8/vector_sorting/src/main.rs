use std::collections::HashMap;

fn main() {
    let my_vector = vec![1,2,3,4,5,6,7,8,9];
    let result_stats = sort_vector(my_vector);
    println!("And the results are median {} and mode {}", result_stats.median, 
    if result_stats.mode == -1 {
        format!("{}","does not exist")
    } else {
        format!("{}",result_stats.mode)
    })
}

struct Results {
    median: i32,
    mode : i32 
}

impl Results{
    fn pack_results(median : i32, mode : i32) -> Results{
        Results {
            median, mode
        }
    }
}

fn sort_vector(mut v :   Vec<i32>) -> Results{
    let mut occurrences: HashMap<i32, u32> = HashMap::with_capacity(v.len());
    v.sort();
    for number in &v {
        let count = occurrences.entry(*number).or_insert(0);
        *count += 1;
    }
    let mut mode: i32 = -1;
    let mut mode_count : u32 = 1;
    for (key, value) in &occurrences {
        if value > &mode_count {
            mode = *key;
            mode_count = *value;
        }
    }
    let temp = v[v.len()/2];
    Results::pack_results(temp, mode)

}
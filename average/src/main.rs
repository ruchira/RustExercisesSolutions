use std::collections::HashMap;

#[derive(Debug)]
struct Averages {
    mean: f64,
    median: f64,
    mode: Option<Vec<i32>>,
}

fn get_mode(numvec: &Vec<i32>) -> Option<Vec<i32>> {
    let mut histogram: HashMap<i32, u32> = HashMap::new();

    for &num in numvec.iter() {
        let count = histogram.entry(num).or_insert(0);
        *count += 1;
        }
    let mut max_frequency = 0u32;

    for (_, &frequency) in &histogram {
        if frequency > max_frequency {
            max_frequency = frequency;
        }
    }

    let mut same_frequency = true;

    for (_, &frequency) in &histogram {
        if frequency != max_frequency {
            same_frequency = false;
        }
    }

    if same_frequency {
        None
    } else {
        let mut most_frequent_nums: Vec<i32> = Vec::new();
        for (&num, &frequency) in &histogram {
            if frequency == max_frequency {
                most_frequent_nums.push(num);
            }
        }
        Some(most_frequent_nums)
    }
}

fn get_averages(nums: &[i32]) -> Averages {
    let mut numvec: Vec<i32> = Vec::from(nums);
    let mut sum: f64 = 0.0;
    for &num in numvec.iter() {
        sum += num as f64;
    }
    numvec.sort();
    let num_nums = nums.len();
    let mid = num_nums / 2;
    let median =
        if num_nums % 2 == 0 {
            let elt0 = numvec[mid - 1];
            let elt1 = numvec[mid];
            ((elt0 + elt1) as f64) * 0.5
        } else {
            let elt = numvec[mid];
            elt as f64
        };
    let mode = get_mode(&numvec);

    Averages{ mean: sum/(num_nums as f64), median: median, mode: mode }
}

fn main() {
    let num_list1 = [32, 27, -5, 17];
    let num_list2 = [100, 0, 30, 70, 80, 80, 65, 90];
    let num_list3 = [38, -27, 3];
    let num_list4 = [0, 5, 5, 10, 10];

    let averages1 = get_averages(&num_list1);
    println!("First number list: {:?} {:?}", &num_list1, &averages1);
    let averages2 = get_averages(&num_list2);
    println!("Second number list: {:?} {:?}", &num_list2, &averages2);
    let averages3 = get_averages(&num_list3);
    println!("Third number list: {:?} {:?}", &num_list3, &averages3);
    let averages4 = get_averages(&num_list4);
    println!("Fourth number list: {:?} {:?}", &num_list4, &averages4);
}

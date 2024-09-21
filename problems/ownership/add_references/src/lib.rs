#![forbid(unsafe_code)]

#[derive(Debug, Eq, PartialEq)]
pub struct Statistics {
    pub average: i32,
    pub median: i32,
    pub min: i32,
    pub max: i32,
}

fn calculate_average(data: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut cnt = 0;
    for x in data {
        sum += x;
        cnt += 1;
    }
    return sum / cnt;
}

fn calculate_median(data: Vec<i32>) -> i32 {
    data.sort();
    return data[data.len() / 2];
}

fn calculate_minmax(data: Vec<i32>) -> (i32, i32) {
    let mut min = data[0];
    let mut max = data[0];
    for x in data {
        if min > x {
            min = x;
        }
        if max < x {
            max = x;
        }
    }
    return (min, max);
}

pub fn calculate_statistics(data: Vec<i32>) -> Statistics {
    let average = calculate_average(data);
    let median = calculate_median(data);
    let (min, max) = calculate_minmax(data);

    return Statistics {
        average,
        median,
        min,
        max,
    };
}

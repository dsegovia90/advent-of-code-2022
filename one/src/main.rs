use crate::data::get_data;

mod data;

fn main() {
    let data = get_data();

    let mut sums: Vec<i32> = Vec::new();

    data.iter().enumerate().for_each(|(i, _item)| {
        if i+3 <= data.len() {
            let sum = data.iter().skip(i).take(3).sum::<i32>();
            sums.push(sum);
        }
    });

    let mut count = 0;
    let mut previous_opt = None;
    sums.iter().for_each(|item| {
        if let Some(prev) = previous_opt {
            if item > prev {
                count += 1;
            }
        }

        previous_opt = Some(item);
    });

    println!("{}:{}", sums.len(), data.len());
    println!("count {:?}", count);
}

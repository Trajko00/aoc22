use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|val| val.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;

            while let Some(Some(val)) = it.next() {
                sum = Some(sum.unwrap_or(0) + val);
            }

            sum
        })
        .max();

    println!("max = {max:?}");

    Ok(())
}

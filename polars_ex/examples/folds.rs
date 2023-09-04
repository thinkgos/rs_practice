use color_eyre::Result;
use polars::prelude::*;

fn main() -> Result<()> {
    let df = df![
        "a" => [1, 2, 3],
        "b" => [10, 20, 30],
        "c" => [5, 1, 7],
    ]?;
    println!("origin data:");
    println!("{}", df);

    manual_sum(df.clone())?;
    conditional(df.clone())?;
    folds_and_string_data(df.clone())?;
    Ok(())
}

fn manual_sum(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([
            col("*"),
            fold_exprs(lit(0), |acc, x| Ok(acc + x), [col("a"), col("b")]).alias("sum(a + b)"), // a列元素+b列元素的和
            fold_exprs(lit(0), |acc, x| Ok(acc + x), [col("*")]).alias("sum(*)"), // 所有列元素横向相向
        ])
        .collect()?;
    println!("manual_sum:");
    println!("{}", out);
    Ok(())
}

fn conditional(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .filter(fold_exprs(
            lit(true),
            |acc, x| acc.bitand(&x), // 与所有列进行与操作
            [col("*").gt(1)],        // 过滤所有列, 要求 > 1
        ))
        .collect()?;
    println!("conditional:");
    println!("{}", out);

    Ok(())
}
fn folds_and_string_data(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([concat_str([col("a"), col("b"), col("c")], "")]) // 拼接 a,b,c列
        .collect()?;
    println!("folds_and_string_data:");
    println!("{}", out);

    Ok(())
}

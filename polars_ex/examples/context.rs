use std::ops::Mul;

use color_eyre::Result;
use polars::prelude::*;
use rand::{thread_rng, Rng};

/// 不能随便的使用表达式, 一个表达式需要一个上下文.
/// - selection: df.select([..])
/// - groupy aggregation: df.groupby(..).agg([..])
/// - hstack/ add columns: df.with_columns([..])
fn main() -> Result<()> {
    let mut arr = [0f64; 5];
    thread_rng().fill(&mut arr);

    // 创建数据集, 5行4列
    // shape: (5, 4)
    let df = df! [
        "nrs" => [Some(1), Some(2), Some(3), None, Some(5)],
        "names" => [Some("foo"), Some("ham"), Some("spam"), Some("eggs"), None],
        "random" => arr,
        "groups" => ["A", "A", "B", "C", "B"],
    ]?;
    println!("origin data:");
    println!("{}", &df);

    select_context(df.clone())?;
    with_columns(df.clone())?;
    group_by_context(df.clone())?;

    Ok(())
}
/// ## select context
///
fn select_context(df: DataFrame) -> Result<()> {
    let out = df
        .clone()
        .lazy()
        .select([
            sum("nrs"),
            col("names").sort(false),
            col("names").first().alias("first name"),
            mean("nrs").mul(lit(10)).alias("10xnrs"),
        ])
        .collect()?;
    println!("select_context");
    println!("{}", out);
    Ok(())
}

/// ## Add columns
///
fn with_columns(df: DataFrame) -> Result<()> {
    let out = df
        .clone()
        .lazy()
        .with_columns([
            sum("nrs").alias("nrs_sum"),
            col("random").count().alias("count"),
        ])
        // .select([col("random"), col("count")])
        .collect()?;
    println!("select_context");
    println!("{}", out);

    Ok(())
}

/// ## group context
///
fn group_by_context(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .groupby([col("groups")])
        .agg([
            sum("nrs"),                           // sum nrs by groups
            col("random").count().alias("count"), // count group members
            // sum random where name != null
            col("random")
                .filter(col("names").is_not_null())
                .sum()
                .suffix("_sum"),
            col("names").reverse().alias("reversed names"),
        ])
        .collect()?;

    println!("group_by_context");
    println!("{}", out);

    Ok(())
}

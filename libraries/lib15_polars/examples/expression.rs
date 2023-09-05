use color_eyre::Result;
use polars::{lazy::dsl::col, prelude::*};
use rand::{thread_rng, Rng};

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

    this_is_a_expression_of_series(df.clone())?;
    select(df.clone())?;
    count_unique(df.clone())?;
    various_aggregations(df.clone())?;
    filter_condition(df.clone())?;
    binary_and_modification(df.clone())?;
    window_expression(df.clone())?;
    Ok(())
}

fn this_is_a_expression_of_series(df: DataFrame) -> Result<()> {
    let _out = df.column("nrs")?.sort(false).head(Some(2));

    // println!("sort:");
    // println!("{}", out);
    Ok(())
}

// 选择列显示
fn select(df: DataFrame) -> Result<()> {
    let out = df.lazy().select([col("names"), col("random")]).collect()?;
    println!("select:");
    println!("{}", out);
    Ok(())
}

// 计算唯一值个数
fn count_unique(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([
            col("names").n_unique().alias("unique_names_1"),
            col("names").unique().count().alias("unique_names_2"),
            col("groups").unique().count().alias("unique_groups"),
        ])
        .collect()?;
    println!("count_unique:");
    println!("{}", out);
    Ok(())
}

// 各种不同的聚合计算
fn various_aggregations(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([
            sum("random").alias("sum"),
            min("random").alias("min"),
            max("random").alias("max"),
            col("random").max().alias("other_max"),
            col("random").std(1).alias("std dev"),
            col("random").var(1).alias("variance"),
        ])
        .collect()?;
    println!("various_aggregations:");
    println!("{}", out);

    Ok(())
}

// 计算以"am"结尾的个数
fn filter_condition(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([col("names")
            .filter(col("names").str().contains("am$"))
            .count()])
        .collect()?;
    println!("filter_condition:");
    println!("{}", out);

    Ok(())
}

// 二元操作和修改
// when 需要一个断言, 评估是一series的bool
// then 表示 when 评估为true的, 使用then的值
// otherwise 表示 when 评估为false时使用 otherwise的值
fn binary_and_modification(df: DataFrame) -> Result<()> {
    let out = df
        .clone()
        .lazy()
        .select([
            col("*"),
            when(col("random").gt(0.5)).then(0).otherwise(col("random")) * sum("nrs"),
            (col("random") * sum("nrs")).alias("random * sum(nrs)"),
            sum("nrs").alias("sum(nrs)"),
        ])
        .collect()?;
    println!("binary_and_modification");
    println!("{}", out);
    Ok(())
}

// 窗口表达式
fn window_expression(df: DataFrame) -> Result<()> {
    let out = df
        .lazy()
        .select([
            col("*"), // Select all
            col("random")
                .sum()
                .over([col("groups")])
                .alias("sum[random]/groups"), // 在分组groups上对random进行求和
            col("random")
                .list()
                .over([col("groups")])
                .alias("random/groups"), // 在分组上groups对random进行列表聚合
        ])
        .collect()?;
    println!("window_expression");
    println!("{}", out);
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{
        DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
    };
    use chrono_tz::Asia::Shanghai;

    /// # 时间 struct
    ///     时间格式                        时间                      = chrono 结构
    /// - "2020-04-12"                  => Date                     = NaiveDate
    /// - "22:10"                       => Time                     = NaiveTime
    /// - "2020-04-12 22:10:57"         => Date + Time              = NaiveDateTime
    /// - "2020-04-12 22:10:57+02:00"   => Date + Time + TimeZone   = DateTime<Tz>
    ///
    /// # 错误
    /// ParseError(NotEnough) 表示没有足够信息填充. 比如 date, time, timezone 信息丢失
    /// ParseError(TooShort) 和 ParseError(Invalid) 表示格式不匹配
    /// # 特殊字符串格格式: "%Y-%m-%d %H:%M:%S"
    ///  https://docs.rs/chrono/latest/chrono/format/strftime/index.html

    #[test]
    fn rfc2822() {
        // RFC2822 =  Date + Time + TimeZone
        // DateTime<Tz>

        let date_str = "Tue, 1 Jul 2003 10:52:37 +0200";
        let dt = DateTime::parse_from_rfc2822(date_str).unwrap();

        assert_eq!(
            dt,
            Utc.ymd(2003, 7, 1)
                .and_hms(8, 52, 37)
                .with_timezone(&FixedOffset::east(2 * 3600))
        );
    }

    #[test]
    fn rfc3339() {
        // RFC3339 = Date + Time + TimeZone
        // DateTime<Tz>

        let date_str = "2020-04-12T22:10:57+02:00";
        // convert the string into DateTime<FixedOffset>
        let dt = DateTime::parse_from_rfc3339(date_str).unwrap();
        // convert the string into DateTime<Utc> or other timezone
        let dt_utc = dt.with_timezone(&Utc);

        let got_dt = Utc
            .ymd(2020, 4, 12)
            .and_hms(20, 10, 57)
            .with_timezone(&FixedOffset::east(2 * 3600));
        let got_dt_utc = got_dt.with_timezone(&Utc);
        assert_eq!(dt, got_dt);
        assert_eq!(dt_utc, got_dt_utc);
    }

    #[test]
    fn non_standard() {
        // Date + Time + Timezone (其它或非标准的)
        // DateTime<Tz>

        let date_str = "2020-04-12 22:10:57 +02:00";
        let dt = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z").unwrap();

        assert_eq!(
            dt,
            Utc.ymd(2020, 4, 12)
                .and_hms(20, 10, 57)
                .with_timezone(&FixedOffset::east(2 * 3600))
        );
    }

    #[test]
    fn naive_date_time() {
        // Date + Time (没有时区)
        // NaiveDateTime

        let date_str = "2020-04-12 22:10:57";
        let naive_dt = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap();

        assert_eq!(
            naive_dt,
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 4, 12),
                NaiveTime::from_hms(22, 10, 57)
            )
        );
    }

    #[test]
    fn date() {
        // Date 只有日期, 没有时间
        // NaiveDate

        let date_str = "2020-04-12";
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

        assert_eq!(naive_date, NaiveDate::from_ymd(2020, 4, 12),);
    }

    #[test]
    fn time() {
        // Time 只有时间, 没有日期
        // NaiveTime

        let time_str = "22:10:57";
        let naive_time = NaiveTime::parse_from_str(time_str, "%H:%M:%S").unwrap();
        assert_eq!(naive_time, NaiveTime::from_hms(22, 10, 57))
    }

    #[test]
    fn add_date_time_and_or_time_zone() {
        let date_str = "2020-04-12";
        // 字符串解析成 NaiveDate
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        // 增加时间转换为 NaiveDateTime
        let naive_dt: NaiveDateTime = naive_date.and_hms(0, 0, 0);
        // 增加时区转换为 DateTime<UTC>
        let dt_utc = DateTime::<Utc>::from_utc(naive_dt, Utc);

        let got_naive_date = NaiveDate::from_ymd(2020, 04, 12);
        let got_naive_dt = got_naive_date.and_hms(0, 0, 0);
        let got_dt_utc = DateTime::<Utc>::from_utc(got_naive_dt, Utc);
        assert_eq!(naive_date, got_naive_date);
        assert_eq!(naive_dt, got_naive_dt);
        assert_eq!(dt_utc, got_dt_utc);
    }

    #[test]
    fn date_and_time() {
        // utc 时区时间
        let utc_now = Utc::now();
        // 本地时区时间
        let local_now = Local::now();
        println!("{}", utc_now);
        println!("{}", local_now);
        // FixedOffset 指定任意的时区

        let est = Shanghai.ymd(2022, 11, 1).and_hms(0, 0, 0);
        let got_east_8 = FixedOffset::east(8 * 3600)
            .ymd(2022, 11, 1)
            .and_hms(0, 0, 0);
        assert_eq!(est, got_east_8)
    }
}

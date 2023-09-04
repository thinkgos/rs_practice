use md5::{self, Context};

fn main() {
    let b = b"admin";
    assert_eq!(
        "21232f297a57a5a743894a0e4a801fc3",
        format!("{:x}", md5::compute(b))
    );
    assert_eq!(
        "21232F297A57A5A743894A0E4A801FC3",
        format!("{:X}", md5::compute(b))
    );

    let mut md5_ctx = Context::new();
    md5_ctx.consume(b"adm");
    md5_ctx.consume("in");
    assert_eq!(
        "21232f297a57a5a743894a0e4a801fc3",
        format!("{:x}", md5_ctx.clone().compute())
    );
    assert_eq!(
        "21232F297A57A5A743894A0E4A801FC3",
        format!("{:X}", md5_ctx.clone().compute())
    );
}

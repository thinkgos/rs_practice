/// Path 和 PathBuf 的关系 相当于 str 和 String
/// Path 路径的切片, 是个 unsize 类型, 所以你应该使用 & 或 Box
/// PathBuf 拥有所有权, 可变的路径

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::Component;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn path_it_work() {
        let path = Path::new("/tmp/foo/bar.txt");
        let parent = path.parent();
        assert_eq!(parent, Some(Path::new("/tmp/foo")));

        // 提取非扩展
        let file_stem = path.file_stem();
        assert_eq!(file_stem, Some(OsStr::new("bar")));

        // 提取扩展
        let extension = path.extension();
        assert_eq!(extension, Some(OsStr::new("txt")));

        // 提取文件名
        let filename = path.file_name();
        assert_eq!(filename, Some(OsStr::new("bar.txt")));
    }

    #[test]
    fn path_buf_it_work() {
        // This way works...
        let mut path = PathBuf::from("/usr");

        path.push("local");
        path.push("bin");

        // ... but push is best used if you don't know everything up
        // front. If you do, this way is better:
        let path1: PathBuf = ["/usr", "local", "bin"].iter().collect();
        assert_eq!(path1, path);
    }
    #[test]
    fn component_it_work() {
        let path = Path::new("/tmp/foo/bar.txt");

        for component in path.components() {
            println!("{:?}, {:?}", component, component.as_os_str());
        }
        // output:
        // RootDir
        // Normal("tmp")
        // Normal("foo")
        // Normal("bar.txt")
    }

    #[test]
    fn component_1_it_work() {
        let path = Path::new("/tmp/foo/bar.txt");
        let components = path.components().collect::<Vec<_>>();
        assert_eq!(
            &components,
            &[
                Component::RootDir,
                Component::Normal("tmp".as_ref()),
                Component::Normal("foo".as_ref()),
                Component::Normal("bar.txt".as_ref()),
            ]
        );
    }

    #[test]
    fn ancestors_it_work() {
        let path = Path::new("/foo/bar");

        for ancestor in path.ancestors() {
            println!("{}", ancestor.display());
        }
    }
}

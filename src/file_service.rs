/// 文件服务, 文件合并, 和文件拆分
pub mod file_service {
    use std::{
        collections::BTreeMap,
        fs::{self, File},
        io::{BufReader, BufWriter, Error, Read, Write},
        path::Path,
    };

    use regex::Regex;

    /// path : 待分的文件的路径;
    pub fn file_split(_path: &str) -> Result<(), Error> {
        let file_path = Path::new(_path);
        if !file_path.is_file() || !file_path.exists() {
            eprintln!("path is not a file");
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "path not exists or is not a file",
            ));
        }
        let input = File::open(file_path)?;
        let mut buffer = vec![0; 1024 * 1024 * 10];
        let mut reader = BufReader::new(input);

        let mut part = 1;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let output_file_name = format!("{}_part{}", _path, part);
            let output_file = File::create(output_file_name)?;

            let mut write = BufWriter::new(output_file);

            write.write(&buffer[..bytes_read])?;
            part += 1;
        }
        Ok(())
    }

    /// path : 文件夹的路径地址;
    /// file_name: 生成的文件名字;
    pub fn file_merge(_path: &str, file_name: &str) -> Result<(), Error> {
        let file_path = Path::new(_path);
        if !file_path.exists() || !file_path.is_dir() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "path is not a directory or is noe exists",
            ));
        }
        let mut map_data = BTreeMap::new();
        for entry in fs::read_dir(file_path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                continue;
            }
            let file_path = entry.path();
            if file_is_merge(file_path.as_path()) {
                let file_index = get_file_original_index(file_path.as_path()).unwrap();
                map_data.entry(file_index).or_insert(file_path);
            }
        }
        let mut original_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();
        // 对map_data进行遍历； 从小到大先排序
        for (_, value) in map_data {
            let mut tem_file = File::open(value).expect("read file failed");
            let mut buffer = Vec::new();
            tem_file.read_to_end(&mut buffer).unwrap();
            original_file.write_all(&mut buffer).unwrap();
        }
        Ok(())
    }

    /// 判断是否符合合并的命名格式;
    fn file_is_merge(file_path: &Path) -> bool {
        let re = Regex::new(r".*_part\d+").unwrap();
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .expect("get file name is error");
        re.is_match(file_name)
    }

    /// 获取原始的未分割的名称;
    fn get_file_original_name(file_path: &Path) -> Result<String, Error> {
        let re = Regex::new(r"^(.*)_part\d+").unwrap();
        let file_name = get_file_name_by_path(file_path).unwrap();
        if let Some(caps) = re.captures(file_name.as_str()) {
            return Ok(caps[1].to_string());
        }
        Err(Error::new(std::io::ErrorKind::Other, "invalid file name"))
    }

    /// 获取分割文件的index索引号；
    fn get_file_original_index(file_path: &Path) -> Result<u16, Error> {
        let re = regex::Regex::new(r"^.*_part(\d+)$").unwrap();
        let file_name = get_file_name_by_path(file_path).unwrap();

        if let Some(caps) = re.captures(file_name.as_str()) {
            let index_str = caps[1].to_string();
            match index_str.parse::<u16>() {
                Ok(value) => {
                    return Ok(value);
                }
                Err(_) => {
                    return Err(Error::new(std::io::ErrorKind::Other, "invalid file name"));
                }
            }
        }
        Err(Error::new(std::io::ErrorKind::Other, "invalid file name"))
    }

    fn get_file_name_by_path(file_path: &Path) -> Result<String, Error> {
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .expect("get file name is error");
        Ok(file_name.to_string())
    }

    #[test]
    fn merge_file_test() {
        file_merge("./", "./test_name.exe").unwrap();
        let file = Path::new("./jdk-8u361-windows-x64.exe");
        assert!(file.exists())
    }

    #[test]
    fn get_file_original_index_test() {
        let _path = Path::new("./jdk-8u361-windows-x64.exe_part1");
        let index = get_file_original_index(_path).unwrap();
        assert!(index == 1);
    }

    #[test]
    fn file_split_test() {
        file_split("./jdk-8u361-windows-x64.exe").expect("file split is failed");
    }

    #[test]
    fn file_merge_test() {
        file_merge("./", "test.aaa.exe").expect("file merge failed");
    }

    #[test]
    fn file_is_merge_test() {
        assert!(file_is_merge(Path::new("./jdk-8u361-windows-x64_part1")));
        assert!(!file_is_merge(Path::new("./jdk-8u361-windows-x64")));
    }

    #[test]
    fn get_file_original_name_test() {
        let path = Path::new("./jdk-8u361-windows-x64.exe_part1");
        let original_name = get_file_original_name(path).unwrap();
        assert_eq!("jdk-8u361-windows-x64.exe", original_name)
    }
}

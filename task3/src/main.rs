use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader, BufWriter};
use std::io::Read;

/// 模块 1：定义数据结构并实现序列化和反序列化
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn serialize_person(person: &Person) -> Result<String, serde_json::Error> {
    serde_json::to_string(person)
}

fn deserialize_person(data: &str) -> Result<Person, serde_json::Error> {
    serde_json::from_str(data)
}

/// 模块 2：编写测试用例和错误处理
fn test_serialization(person: &Person) {
    match serialize_person(person) {
        Ok(serialized) => {
            println!("Serialized JSON: {}", serialized);
            match deserialize_person(&serialized) {
                Ok(deserialized) => assert_eq!(person, &deserialized),
                Err(e) => eprintln!("Deserialization error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Serialization error: {:?}", e),
    }
}

/// 模块 3：读取文件并统计单词数量
fn count_words_in_file(filename: &str) -> io::Result<HashMap<String, usize>> {
    let file_content = fs::read_to_string(filename)?;
    let mut word_count = HashMap::new();

    for word in file_content.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }
    Ok(word_count)
}

fn write_word_count_to_file(word_count: &HashMap<String, usize>, output_filename: &str) -> io::Result<()> {
    let mut output_file = File::create(output_filename)?;
    for (word, count) in word_count {
        writeln!(output_file, "{}: {}", word, count)?;
    }
    Ok(())
}

/// 模块 4：读取并解析二进制文件内容
#[derive(Debug, Clone)]
struct Record {
    record_type: u8,
    id: u32,
    name: String,
}

fn parse_binary_file(filename: &str) -> io::Result<Vec<Record>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut records = Vec::new();

    loop {
        let mut type_buf = [0; 1];
        if reader.read_exact(&mut type_buf).is_err() {
            break;  // 文件结束
        }
        let record_type = type_buf[0];

        let mut id_buf = [0; 4];
        reader.read_exact(&mut id_buf)?;
        let id = u32::from_le_bytes(id_buf);

        let mut name_buf = Vec::new();
        reader.read_until(0, &mut name_buf)?;
        name_buf.pop();
        let name = String::from_utf8(name_buf).expect("Invalid UTF-8");

        records.push(Record { record_type, id, name });
    }

    Ok(records)
}

/// 模块 5：记录过滤和排序功能
fn filter_records(records: &[Record], record_type_filter: Option<u8>, id_threshold: Option<u32>) -> Vec<Record> {
    records.iter()
        .filter(|record| {
            let type_match = record_type_filter.map_or(true, |t| record.record_type == t);
            let id_match = id_threshold.map_or(true, |id| record.id > id);
            type_match && id_match
        })
        .cloned()
        .collect()
}

/// 按记录ID排序
fn sort_records_by_id(records: &mut Vec<Record>) {
    records.sort_by_key(|record| record.id);
}

/// 模块 6：汇总统计信息
/// 统计每种 record_type 的数量
fn display_statistics(records: &[Record]) {
    let mut type_count = HashMap::new();
    for record in records {
        *type_count.entry(record.record_type).or_insert(0) += 1;
    }

    println!("\nRecord Type Statistics:");
    for (record_type, count) in &type_count {
        println!("Type {}: {} records", record_type, count);
    }
}

/// 主函数
fn main() -> io::Result<()> {
    // 示例数据
    let person = Person {
        name: String::from("shalins"),
        age: 18,
        email: String::from("shalins@example.com"),
    };

    // 任务 1 测试序列化和反序列化
    test_serialization(&person);

    // 任务 2 读取文件并统计单词
    let word_count = count_words_in_file("src/word.txt")?;
    write_word_count_to_file(&word_count, "src/word_count.txt")?;

    // 创建一个 data.bin 文件
    let file = File::create("src/data.bin")?;  // 注意文件路径
    let mut writer = BufWriter::new(file);

    // 构造一些示例记录
    let records = vec![
    (1u8, 100u32, "Alice"),
    (2u8, 200u32, "Bob"),
    (1u8, 150u32, "Charlie"),
    (3u8, 300u32, "Diana"),
];

    // 将记录写入二进制文件
    for (record_type, id, name) in records {
        // 写入记录类型
        writer.write_all(&[record_type])?;
        
        // 写入ID，转换为小端序
        writer.write_all(&id.to_le_bytes())?;
        
        // 写入名称并以 0x00 结尾
        writer.write_all(name.as_bytes())?;
        writer.write_all(&[0x00])?;
    }

    writer.flush()?; // 确保所有数据写入文件
    println!("src/data.bin 文件已成功生成！");

    // 任务 3 解析二进制文件并输出
    let records = parse_binary_file("src/data.bin")?;

    // 使用过滤和排序功能
    let filtered_records = filter_records(&records, Some(1), Some(100)); // 仅显示类型1并且ID > 100的记录
    let mut records_to_display = filtered_records.clone();
    sort_records_by_id(&mut records_to_display);

    // 输出过滤后的记录
    println!("\nFiltered and Sorted Records:");
    for record in &records_to_display {
        println!("{:?}", record);
    }

    // 显示汇总统计信息
    display_statistics(&records_to_display);

    Ok(())
}


use std::{io::{self, Write, stdin}, fmt::Display};
use rand::Rng;
use std::cmp::Ordering;
const NAMES_CONTENT: &'static str = include_str!("./name.txt");

fn main() {
    let mut records = ExamRecord::gen_records();
    loop {
        print_main_menu();
        // println!("{:?}", records);
        print!("\n请选择 [1-5]: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "5" => break,
            "1" => {
                print_records(&mut records);
            },
            "2" => {
                // sort_by_total_score(&mut records);
                // print_records(&mut records)
                sort(&mut records)
            },
            _ => println!("Unknow Option")
        }
    }

    

}
fn print_records(records: &mut Vec<ExamRecord>) {
    println!("  #       id    姓名    语文    数学    英语     总分   平均分");
    records.iter().for_each(|exam_record| {
        println!("{:03} {}", exam_record.id, exam_record);
    })
}
fn sort(records: &mut Vec<ExamRecord>) {
    println!("输入1按照总成绩排序");
    println!("输入2按照平均分排序");
    println!("输入3按照语文成绩排序");
    println!("输入4按照数学成绩排序");
    println!("输入5按照英语成绩排序");
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => {
            let mut new_records = records.clone();
            sort_by_total_score(&mut new_records);
            print_records(&mut new_records);
        },
        "2" => {
            let mut new_records = records.clone();
            sort_by_avag(&mut new_records);
            print_records(&mut new_records);
        },
        "3" => {
            let mut new_records = records.clone();
            sort_by_chinese(&mut new_records);
            print_records(&mut new_records);
        },
        "4" => {
            let mut new_records = records.clone();
            sort_by_math(&mut new_records);
            print_records(&mut new_records);
        },
        "5" => {
            let mut new_records = records.clone();
            sort_by_english(&mut new_records);
            print_records(&mut new_records);
        },
        _ => {
            println!("Unknow Option")
        }
    }
}
fn sort_by_chinese(records: &mut Vec<ExamRecord>) {
    records.sort_by(|a, b| {
        match (a.scores[0], b.scores[0]) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    })
}
fn sort_by_math(records: &mut Vec<ExamRecord>) {
    records.sort_by(|a, b| {
        match (a.scores[1], b.scores[1]) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    })
}
fn sort_by_english(records: &mut Vec<ExamRecord>) {
    records.sort_by(|a, b| {
        match (a.scores[2], b.scores[2]) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    })
}

fn sort_by_total_score(records: &mut Vec<ExamRecord>) {
    records.sort_by(|a, b| {
        match (a.total_score, b.total_score) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    })
}

fn sort_by_avag(records: &mut Vec<ExamRecord>) {
    records.sort_by(|a, b| {
        match (a.avg_score, b.avg_score) {
            (Some(sa), Some(sb)) => sb.total_cmp(&sa),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    })
}
fn print_main_menu() {
    println!("\n------ 成绩查询系统 ------");
    println!("  1. 打印成绩单");
    println!("  2. 排序成绩单");
    println!("  3. 查找成绩单");
    println!("  4. 全班平均成绩");
    println!("  5. 退出");
}

#[derive(Debug, Clone)]
struct ExamRecord {
    id: u64,
    name: String,
    scores: Vec<Option<f32>>,
    total_score: Option<f32>,
    avg_score: Option<f32>
}
impl ExamRecord {
    fn gen_records() -> Vec<ExamRecord> {
        // ...
        let mut id = 0u64;
        NAMES_CONTENT.split("\n").filter(|s| !s.is_empty()).take(100).map(|s| {
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(1..=100);
            let chinese_score = if (id + 1) % 10 == 0 && n % 7 == 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + 60.0 * score;
                Some(score)
            };
            let math_score = if (id + 1) % 10 == 0 && n % 11 == 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + 60.0 * score;
                Some(score)
            };

            let english_score = if (id + 1) % 10 == 0 && n % 11 == 0 {
                None
            } else {
                let score: f32 = rng.gen();
                let score = 40.0 + 60.0 * score;
                Some(score)
            };

            let scores = vec![chinese_score, math_score, english_score];
            let attend_exams: Vec<&Option<f32>> = scores.iter().filter(|s| s.is_some()).collect();
            
            let (total_score, avg_score) = if attend_exams.is_empty() {
                (None, None)
            } else {
                let total = attend_exams.iter().map(|s| s.unwrap()).sum();
                (Some(total), Some(total / attend_exams.len() as f32))
            };
            id += 1;
            ExamRecord {
                id,
                name: s.to_string(),
                scores,
                total_score,
                avg_score
            }
        }).collect()
    }
}

impl Display for ExamRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "2024-{:03}  {}  {:>6}  {:>6}  {:>6}  {:>7}  {:>6}", 
            self.id,
            self.name,
            match self.scores[0] {
                Some(score)  => format!("{:.2}", score),
                None => "-".to_string()
            },
            match self.scores[1] {
                Some(score)  => format!("{:.2}", score),
                None => "-".to_string()
            },
            match self.scores[2] {
                Some(score)  => format!("{:.2}", score),
                None => "-".to_string()
            },
            match self.total_score {
                Some(score)  => format!("{:.2}", score),
                None => "-".to_string()
            },
            match self.avg_score {
                Some(score)  => format!("{:.2}", score),
                None => "-".to_string()
            },
        )
    }
}
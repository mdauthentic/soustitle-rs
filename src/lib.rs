use std::fs;
use std::fs::File;
use std::fmt;
use std::path::Path;
use std::io::Write;
use std::error::Error;
use chrono::NaiveTime;

const SRT_MIN_LEN: usize = 3;
const SRT_CSV_HEADER: &str = "id,start_time,end_time,text";

pub struct Subtitle {
    pub id: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub text: String,
}

impl Subtitle {
    fn new(id: i32, start_time: NaiveTime, end_time: NaiveTime, text: String) -> Subtitle {
        Subtitle{
            id: id, 
            start_time: start_time, 
            end_time: end_time, 
            text: text,
        }
    }

    fn srt_text(&self) -> String {
        return format!("{}", self.text);
    }

    fn as_csv_str(&self) -> String {
        return format!("{},{},{},{}", self.id, self.start_time, self.end_time, self.text);
    }
}

impl fmt::Debug for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subtitle")
         .field("id", &self.id)
         .field("start_time", &self.start_time)
         .field("end_time", &self.end_time)
         .field("text", &self.text)
         .finish()
    }
}

pub fn parse<P: AsRef<Path>>(f_path: P) -> Vec<Subtitle> {
    let raw_data = read_file(f_path).unwrap();
    return parse_srt_string(string_to_static_str(raw_data));
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn write_data<P: AsRef<Path>>(srt_data: Vec<Subtitle>, file_name: P, all_fields: bool) -> std::io::Result<()> {
    let mut f = File::create(file_name).unwrap();

    if all_fields {
        writeln!(&mut f, "{}", SRT_CSV_HEADER).unwrap();
        for item in &srt_data {
            writeln!(&mut f, "{}", item.as_csv_str()).unwrap();
        }
    } else {
        for item in &srt_data {
            writeln!(&mut f, "{}", item.srt_text()).unwrap();
        }
    }

    Ok(())
}

fn str_to_time(raw_time: &str) -> NaiveTime {
    return NaiveTime::parse_from_str(raw_time, "%H:%M:%S:%3f").unwrap();
}

pub fn parse_srt_string(srt: &'static str) -> Vec<Subtitle> {

    let split_srt: Vec<&str> = srt.split("\n\n").collect();

    let mut content: Vec<Subtitle> = Vec::new();

    for item in split_srt.iter() {
        let srt_instance: Vec<&str> = item.split("\n").collect();

        if srt_instance.len() >= SRT_MIN_LEN {
            let srt_time: Vec<&str> = srt_instance[1].split(" --> ").collect();
            let time_start = srt_time[0].replace(",", ":");
            let time_end = srt_time[1].replace(",", ":");

            let mut subtitle = String::new();
            for x in 2..srt_instance.len() {
                let srt_with_space = format!("{} {}", srt_instance[x], " ");
                subtitle.push_str(&srt_with_space);
            }
            
            content.push(Subtitle::new(
                    srt_instance[0].parse::<i32>().unwrap(), 
                    str_to_time(&time_start), 
                    str_to_time(&time_end), 
                    subtitle
                )
            );
        } else {
            continue;
        }
    }

    return content;
}
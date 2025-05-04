use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::process;
#[derive(Clone)]
//this is where I create the Student struct to shove all of the values from the csv into and read out the csv

pub struct Student {
    pub id: i32,
    pub gender: f32,
    pub age: f32,
    pub city: String,
    pub profession: String,
    pub academic_pressure: f32,
    pub work_pressure: f32, //I think all the values for this are 0? but the kaggle says it's in a range from 0-.25 so im setting it as a f32 just in case
    pub cgpa: f32,
    pub study_satisfaction: f32,
    pub job_satisfaction: f32,
    pub sleep_duration: f32, //just changing this to a f32 value instead of a string
    pub dietary_habits: f32, //changing this too
    pub degree: String,
    pub suicidal_thoughts: f32,
    pub workstudy_hours: f32,
    pub financial_stress: f32,
    pub family_history: f32,
    pub depression: f32,
}

pub fn parse_gender(s: &str) -> f32 { //these are all just to turn the entries in the csv into floats if they have something other than numerical values
    match s.trim().to_lowercase().as_str() {
        "Male" => 1.0,
        "Female" => 0.0,
        _ => 0.5
    }
}

pub fn parse_sleep_duration(s: &str) -> f32 { 
    match s.trim().to_lowercase().as_str() {
        "'less than 5 hours'" => 4.5,
        "'5-6 hours'" => 5.5,
        "'6-7 hours'" => 6.5,
        "'7-8 hours'" => 7.5,
        "'more than 8 hours'" => 9.0,
        _ => 0.0
    }
}

pub fn parse_dietary_habits(s: &str) -> f32 {
    match s.trim().to_lowercase().as_str() {
        "Healthy" => 2.0,
        "Moderate" => 1.0,
        "Unhealthy" => 0.0,
        _ => 1.0
    }
}

pub fn parse_fam_history(s: &str) -> f32 { //this and the next one are the same. I should've just generalized it into a simple "parse_yes_no" but it works so I'm not touching it anymore
    match s.trim().to_lowercase().as_str() {
        "Yes" => 1.0,
        "No" => 0.0,
        _ => 0.0
    }
}

pub fn parse_suicidethoughts(s: &str) -> f32 { 
    match s.trim().to_lowercase().as_str() {
        "Yes" => 1.0,
        "No" => 0.0,
        _ => 0.0
    }
}





pub fn csvread(path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
    .delimiter(b',')
    .has_headers(true)
    .from_path(path);

    let mut data = Vec::new();
    for result in rdr?.records(){
        let r = result.unwrap();

        let student = Student{
            id: r[0].parse().unwrap_or(0),
            gender: parse_gender(&r[1]), //not using this
            age: r[2].parse().unwrap_or(0.0),
            city: r[3].to_string(), //not using
            profession: r[4].to_string(), //or this
            academic_pressure: r[5].parse().unwrap_or(0.0),
            work_pressure: r[6].parse().unwrap_or(0.0),
            cgpa: r[7].parse().unwrap_or(0.0),
            study_satisfaction: r[8].parse().unwrap_or(0.0),
            job_satisfaction: r[9].parse().unwrap_or(0.0),
            sleep_duration: parse_sleep_duration(&r[10]),
            dietary_habits: parse_dietary_habits(&r[11]),
            degree: r[12].to_string(), //or this
            suicidal_thoughts: parse_suicidethoughts(&r[13]),
            workstudy_hours: r[14].parse().unwrap_or(0.0),
            financial_stress: r[15].parse().unwrap_or(0.0),
            family_history: parse_fam_history(&r[16]),
            depression: r[17].parse().unwrap_or(0.0),
        };
        data.push(student);

    }
    Ok(data)


}


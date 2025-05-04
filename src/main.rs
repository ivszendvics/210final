use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::process;
mod csvread;
use crate::csvread::Student;

fn targets_features(data: Vec<Student>) -> Result<(Vec<Vec<f32>>, Vec<f32>),Box<dyn Error>>{
    let mut targets = vec![];
    let mut features = vec![];

    for value in data{
        features.push(vec![
            value.age,
            value.academic_pressure,
            value.work_pressure,
            value.cgpa,
            value.study_satisfaction,
            value.job_satisfaction,
            value.sleep_duration,
            value.dietary_habits,
            value.suicidal_thoughts,
            value.workstudy_hours,
            value.financial_stress,
            value.family_history
        ]);
        targets.push(value.depression);
    }
    Ok((features,targets))
}


fn main() {
    let data = csvread::csvread("student_depression_dataset.csv");




    

}

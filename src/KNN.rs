use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::process;
use crate::csvread::Student;
//this module is where I do the actual knn calculation - the targets_features function is to separate the data in the vector of Students into a vector of vector of values in the dataset for each label and a vector of targets (whether or not theyre depressed)
//distance is just a helper function to find the distances between the points in the test and train features

pub fn targets_features(data: Vec<Student>) -> Result<(Vec<Vec<f32>>, Vec<u8>), Box<dyn Error>> {
    let mut features = vec![];
    let mut targets = vec![];

    for value in data {
        features.push(vec![
            value.gender,
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
            value.family_history,
        ]);

        
        targets.push(value.depression as u8);
    }

    Ok((features, targets))
}


pub fn distance(x1: &[f32], x2: &[f32]) -> f32{ //distance between two points, simple enough
    x1.iter().zip(x2.iter()).map(|(a, b)| (a - b).powf(2.0)).sum::<f32>().sqrt() //need to use .sum to add every squared difference together before square rooting all of them
}

pub fn knn(traindata: Vec<Student>, testdata: Vec<Student>, k: usize) -> Vec<u8> {
    let (trainfeatures, traintargets) = targets_features(traindata).unwrap();
    let (testfeatures,testtargets) = targets_features(testdata).unwrap();

    testfeatures.iter().map(|test_point| { //iterate over every testfeature
        let mut distances: Vec<(f32, u8)> = trainfeatures.iter()
            .zip(&traintargets) //zips together trainfeatures and traintargets into one iterator
            .map(|(train_point, &label)| (distance(test_point, train_point), label)) //map the distance between test and train points and the label and put it into tuple/iterator
            .collect();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); //sort by distance

        let mut count_0 = 0; //number of zeroes (depression)
        let mut count_1 = 0; //number of ones (no depression)
        for &(burger, label) in distances.iter().take(k) { //iterates over first k values (k neighbors) in sorted distances
            if label == 1 { count_1 += 1; } else { count_0 += 1; } //checks if 0 or 1 (depression value) and shoves into count
        }

        if count_1 > count_0 { 1
         } else { 0
         } //assigns either 1 or 0 depending on whether there are more 1s or 0s in the closest k neighbors. if it's a tie it'll assign 0 (yes depression) but we're making k an odd number anyways
    }).collect() //shoves every prediction into the vec

}


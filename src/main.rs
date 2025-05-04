use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::process;
mod csvread;
mod KNN;
use crate::csvread::Student;
//this is where we actually read out the csv, call KNN and figure out the accuracy




fn main() {
    let mut data = csvread::csvread("student_depression_dataset.csv").expect("error");



    data.shuffle(&mut thread_rng()); //mixing the Vec<Student> randomly


    let split_index = (data.len() as f32 * 0.9).round() as usize; //finding index to split the the data into training and test data (im just using a .9-.1 split)

    let (train_data, test_data) = data.split_at(split_index); //splitting the mixed data

    let predicted = KNN::knn(train_data.to_vec(), test_data.to_vec(), 101); //don't actually really know why I need to_vec() here but it spat an error at me

    let (test_features, test_targets) = KNN::targets_features(test_data.to_vec()).unwrap(); //same here but it works

    let correct = predicted.iter().zip(test_targets.iter()).filter(|(a, b)| a == b).count(); //zipping together the predicted labels and the actual test targets, and then checking if they're equal to count number of correct guesses



    let accuracy = correct as f32 / test_targets.len() as f32; //finding fraction correct
    println!("Accuracy: {:?}", accuracy * 100.0); //acc


    

}


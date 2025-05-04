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

pub fn distance(x1: f32, x2:f32) -> f32{
    let subtract = x1-x2;
    let squared = subtract*subtract;
    let sqrt = squared.sqrt();
    sqrt;
}

pub fn knn(trainx: &Vec<Vec<f32>>,trainy: &Vec<Vec<f32>>, testy:)

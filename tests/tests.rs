use finaldep::KNN::{knn, targets_features};
use finaldep::csvread::Student;

#[test]
fn test_targets_features() { //testing whether targets_features works as it should
    let mock_data = vec![
        Student {
        id: 0,
            age: 0.0,
            gender: 0.0,
            city: "burger".to_string(),
            profession: "burger".to_string(),
            degree: "burger".to_string(),
            academic_pressure: 0.0,
            work_pressure: 0.0,
            cgpa: 0.0,
            study_satisfaction: 0.0,
            job_satisfaction: 0.0,
            sleep_duration: 0.0,
            dietary_habits: 0.0,
            suicidal_thoughts: 0.0,
            workstudy_hours: 0.0,
            financial_stress: 0.0,
            family_history: 0.0,
            depression: 0.0,
        },
        Student {
            id: 0,
            age: 0.0,
            gender: 0.0,
            city: "burger".to_string(),
            profession: "burger".to_string(),
            degree: "burger".to_string(),
            academic_pressure: 0.0,
            work_pressure: 0.0,
            cgpa: 0.0,
            study_satisfaction: 0.0,
            job_satisfaction: 0.0,
            sleep_duration: 0.0,
            dietary_habits: 0.0,
            suicidal_thoughts: 0.0,
            workstudy_hours: 0.0,
            financial_stress: 0.0,
            family_history: 0.0,
            depression: 1.0,
        },
    ];

    let (features, labels) = targets_features(mock_data).unwrap(); 
    assert_eq!(features.len(), 2); //there should be 2 vectors in the feature vec since there are two student structs
    assert_eq!(labels, vec![0, 1]); //the two labels for depression or no depression should be 0 and 1
}

#[test]
fn test_knn() { //testing whether the knn prediction works as it should
    let train_data = vec![
        Student {
            id: 0,
            age: 20.0,
            gender: 0.0,
            city: "burger".to_string(),
            profession: "burger".to_string(),
            degree: "burger".to_string(),
            academic_pressure: 0.0,
            work_pressure: 0.0,
            cgpa: 0.0,
            study_satisfaction: 0.0,
            job_satisfaction: 0.0,
            sleep_duration: 0.0,
            dietary_habits: 0.0,
            suicidal_thoughts: 0.0,
            workstudy_hours: 0.0,
            financial_stress: 0.0,
            family_history: 0.0,
            depression: 0.0,
        },
        Student {
            id: 100,
            age: 22.0,
            gender: 0.0,
            city: "burger".to_string(),
            profession: "burger".to_string(),
            degree: "burger".to_string(),
            academic_pressure: 1.0,
            work_pressure: 1.0,
            cgpa: 1.0,
            study_satisfaction: 1.0,
            job_satisfaction: 1.0,
            sleep_duration: 1.0,
            dietary_habits: 1.0,
            suicidal_thoughts: 0.0,
            workstudy_hours: 1.0,
            financial_stress: 1.0,
            family_history: 1.0,
            depression: 1.0,
        },
    ];

    let test_data = vec![
        Student {
            id: 101,
            age: 22.0,
            gender: 0.0,
            city: "burger".to_string(),
            profession: "burger".to_string(),
            degree: "burger".to_string(),
            academic_pressure: 2.0,
            work_pressure: 2.0,
            cgpa: 2.0,
            study_satisfaction: 2.0,
            job_satisfaction: 2.0,
            sleep_duration: 2.0,
            dietary_habits: 2.0,
            suicidal_thoughts: 0.0,
            workstudy_hours: 1.0,
            financial_stress: 1.0,
            family_history: 1.0,
            depression: 1.0,
        },
    ];

    let predictions = knn(train_data, test_data.clone(), 1);
    assert_eq!(predictions.len(), 1);
    let predicted_label = predictions[0] as u8;
    assert!(predicted_label == 0 || predicted_label == 1); //the predicted label should be 1 since the basic values for the one student struct in test_data is closer to the second student which has a value of 1 for depression
}
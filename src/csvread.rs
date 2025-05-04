pub fn csvread(path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
    .delimiter(b',')
    .has_headers(true)
    .from_path(path);

    let mut data = Vec::new();
    for result in rdr.records(){
        let r = result.unwrap();

        let student = Student{
            id: r[0].parse().unwrap(),
            gender: r[1].to_string(),
            age: r[2].parse().unwrap(),
            city: r[3].to_string(),
            profession: r[4].to_string(),
            academic_pressure: r[5].parse().unwrap(),
            work_pressure: r[6].parse().unwrap(),
            cgpa: r[7].parse().unwrap(),
            study_satisfaction: r[8].parse().unwrap(),
            job_satisfaction: r[9].parse().unwrap(),
            sleep_duration: r[10].
        }

    }




}
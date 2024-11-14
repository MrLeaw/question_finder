use std::io::Write;

use colored::Colorize;
use csv::Reader;
use dirs::home_dir;

#[derive(Debug, Clone)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct_answer: usize, // (0, 1, 2, 3)
}

fn load_data() -> Vec<Question> {
    let home = home_dir().expect("Unable to find home directory");
    let path = "/app/q.csv";
    let path = home.to_str().unwrap().to_string() + path;
    let data = std::fs::read_to_string(path).expect("Unable to read file");
    let mut questions: Vec<Question> = Vec::new();
    let mut rdr = Reader::from_reader(data.as_bytes());
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        let q = Question {
            question: record[0].to_string(),
            answers: vec![
                record[1].to_string(),
                record[2].to_string(),
                record[3].to_string(),
                record[4].to_string(),
            ],
            correct_answer: record[5].parse::<usize>().unwrap() - 1,
        };
        questions.push(q);
    }
    questions
}

fn add_to_csv(q: Question) {
    let home = home_dir().expect("Unable to find home directory");
    let path = "/app/q.csv";
    let path = home.to_str().unwrap().to_string() + path;
    // append, don't overwrite
    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Unable to open file");
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&[
        q.question,
        q.answers[0].to_string(),
        q.answers[1].to_string(),
        q.answers[2].to_string(),
        q.answers[3].to_string(),
        (q.correct_answer + 1).to_string(),
    ])
    .expect("Unable to write record");

    wtr.flush().expect("Unable to flush");
}

fn remove_from_csv(question: Question) {
    let home = home_dir().expect("Unable to find home directory");
    let path = "/app/q.csv";
    let path = home.to_str().unwrap().to_string() + path;
    let current_data = load_data();
    // we need to match the question, the answers, and the correct answer
    // only drop if all three match
    let current_data = current_data
        .iter()
        .filter(|q| {
            q.question != question.question
                || q.answers != question.answers
                || q.correct_answer != question.correct_answer
        })
        .collect::<Vec<_>>();
    let mut writer = csv::Writer::from_path(path).expect("Unable to write to file");
    // write header: Question,Answer1,Answer2,Answer3,Answer4,Correct Answer
    writer
        .write_record(&[
            "Question",
            "Answer1",
            "Answer2",
            "Answer3",
            "Answer4",
            "Correct Answer",
        ])
        .expect("Unable to write record");
    for q in current_data {
        writer
            .write_record(&[
                q.question.to_string(),
                q.answers[0].to_string(),
                q.answers[1].to_string(),
                q.answers[2].to_string(),
                q.answers[3].to_string(),
                (q.correct_answer + 1).to_string(),
            ])
            .expect("Unable to write record");
    }
    writer.flush().expect("Unable to flush");
}

fn print_init_text(q_count: usize, clear_screen: bool) {
    if clear_screen {
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("Loaded {} questions", q_count.to_string().bright_yellow());
    println!("Type {} to add a new question", "add".bright_green());
    println!("Type {} to remove a question", "remove".bright_red());
    println!("Type {} to quit\n\n", "exit".bright_purple());
}

fn main() {
    let mut data = load_data();
    print_init_text(data.len(), true);
    loop {
        let mut query = String::new();
        print!("{}", "Search: ".bright_blue());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();
        if query == "exit" {
            break;
        }
        if query == "add" {
            // ask for question
            println!("\n\n{}", "Add a new question".bright_cyan());
            print!("{}", "Question: ".bright_blue());
            std::io::stdout().flush().unwrap();
            let mut question = String::new();
            std::io::stdin().read_line(&mut question).unwrap();
            let question = question.trim();
            // ask for answers 1 through 4
            let mut answers: Vec<String> = Vec::new();
            for i in 1..5 {
                print!("{}", format!("Answer {}: ", i).bright_blue());
                std::io::stdout().flush().unwrap();
                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer).unwrap();
                answers.push(answer.trim().to_string());
            }
            // ask for correct answer
            print!("{}", "Correct Answer (1-4): ".bright_blue());
            std::io::stdout().flush().unwrap();
            let mut correct_answer = String::new();
            std::io::stdin().read_line(&mut correct_answer).unwrap();
            let correct_answer = correct_answer.trim().parse::<usize>().unwrap() - 1;
            // append to data
            let question = Question {
                question: question.to_string(),
                answers: answers,
                correct_answer: correct_answer,
            };
            add_to_csv(question);
            println!("{}\n\n\n", "Question added".bright_green());
            data = load_data();
            print_init_text(data.len(), false);
            continue;
        }
        if query == "remove" {
            // ask for question
            println!("\n\n{}", "Remove a question".bright_cyan());
            print!("{}", "Question: ".bright_blue());
            std::io::stdout().flush().unwrap();
            let mut question = String::new();
            std::io::stdin().read_line(&mut question).unwrap();
            // print all matches for the question
            let question = question.trim();
            let matches = data
                .iter()
                .filter(|q| q.question.contains(question))
                .collect::<Vec<_>>();
            let q: Question;
            match matches.len() {
                0 => {
                    println!("{}", "No matches found\n\n".bright_red());
                    print_init_text(data.len(), false);
                    continue;
                }
                1 => {
                    q = matches[0].clone();
                }
                _ => {
                    println!("{}\n\n", "Multiple matches found".bright_yellow());
                    for (i, q) in matches.iter().enumerate() {
                        println!("{}: {}", format!("{}", i).bright_blue(), q.question);
                        println!("Answers: {}", q.answers.join(", "));
                        println!("Correct Answer: {}\n", q.correct_answer + 1);
                    }
                    print!("{}", "Select a question: ".bright_blue());
                    std::io::stdout().flush().unwrap();
                    let mut selection = String::new();
                    std::io::stdin().read_line(&mut selection).unwrap();
                    let selection = selection.trim().parse::<usize>().unwrap();
                    q = matches[selection].clone();
                }
            }
            // ask for confirmation
            println!(
                "{}",
                "Are you sure you want to remove this question?".bright_red()
            );
            println!("{}", q.question);
            println!("Answers: {}", q.answers.join(", "));
            println!("Correct Answer: {}\n", q.correct_answer + 1);
            print!("{}", "Confirm (y/n): ".bright_blue());
            std::io::stdout().flush().unwrap();
            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).unwrap();
            let confirm = confirm.trim();
            if confirm == "y" {
                remove_from_csv(q);
                println!("{}\n\n\n", "Question removed".bright_green());
                data = load_data();
                print_init_text(data.len(), false);
                continue;
            } else {
                println!("{}\n\n\n", "Question not removed".bright_yellow());
                print_init_text(data.len(), false);
                continue;
            }
        }
        let matches = data
            .iter()
            .filter(|q| q.question.contains(query))
            .collect::<Vec<_>>();

        if matches.len() == 0 {
            println!("{}", "No matches found".bright_red());
        } else {
            let first = matches[0];
            let answers_match: bool = matches
                .iter()
                .all(|q| q.correct_answer == first.correct_answer);
            if answers_match {
                println!("{}", first.answers[first.correct_answer].bright_green());
            } else {
                let questions_match: bool =
                    matches.iter().all(|q| q.question == matches[0].question);
                if questions_match {
                    for res in matches {
                        println!("{}\n", res.answers[res.correct_answer].bright_green());
                    }
                } else {
                    for res in matches {
                        println!(
                            "{}\nQ: {}\n",
                            res.answers[res.correct_answer].bright_green(),
                            res.question,
                        );
                    }
                }
            }
        }

        // find all questions where one of the answers includes the query
        let matches = data
            .iter()
            .filter(|q| q.answers.iter().any(|a| a.contains(query)))
            .collect::<Vec<_>>();

        if matches.len() != 0 {
            println!("{}\n\n", "Answer Matches: ".bright_yellow());
            for res in matches {
                let default_answer = String::new();
                let matched_answer = res
                    .answers
                    .iter()
                    .filter(|a| a.contains(query))
                    .next()
                    .unwrap_or(&default_answer);
                println!(
                    "{}\nQ: {} | MA: {}\n",
                    res.answers[res.correct_answer as usize].bright_green(),
                    res.question,
                    matched_answer
                );
            }
        }

        println!("\n\n\n\n");
    }
}

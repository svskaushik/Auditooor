extern crate colored;
use std::io::Write;
use fancy_regex::Regex;
use walkdir::{WalkDir};
use colored::*;
use std::fs::File;
use std::fs;
mod regexrules;
use regexrules::RulesDataBase;
use std::time::Instant;



// Struct to store all the vulns/gas optimization and where they occur in the target

// Initialize the struct
impl Default for RulesDataBase {
    fn default () -> RulesDataBase {
        RulesDataBase {id: String::new(), title: String::new(), description: String::new(), location: Vec::new(), rule: String::new(), recommendation: String::new()}
    }
}

static BANNER: &'static str = "
_______  __   __  ______   ___   _______  _______  _______  _______  ______   
|   _   ||  | |  ||      | |   | |       ||       ||       ||       ||    _ |  
|  |_|  ||  | |  ||  _    ||   | |_     _||   _   ||   _   ||   _   ||   | ||  
|       ||  |_|  || | |   ||   |   |   |  |  | |  ||  | |  ||  | |  ||   |_||_ 
|       ||       || |_|   ||   |   |   |  |  |_|  ||  |_|  ||  |_|  ||    __  |
|   _   ||       ||       ||   |   |   |  |       ||       ||       ||   |  | |
|__| |__||_______||______| |___|   |___|  |_______||_______||_______||___|  |_|";

fn main() {
    banner();
    let path_name: String<> = String::from("contracts"); // The directory where the solidity files are located
    let mut gasopresults: Vec<RulesDataBase>;
    let mut lowresults: Vec<RulesDataBase>;
    let mut ncresults: Vec<RulesDataBase>;
    let files_names: Vec<String> = is_solidity_file(&path_name); // Vector with solidity files to analyze

    let gasoprules = regexrules::gas_op_issues();
    let lowrules = regexrules::low_issues();
    let ncrules = regexrules::non_critical_issues();

    println!("\n{}","Solidity files found:");
    for file in &files_names{
        println!("{}", file.on_white().black())
    }
    print!("\n{}","---------Analyzing---------\n\n");

    let gasop_file = std::fs::File::create("Gas-Optimization Results.md").expect("(Writing_to_a_file error) -> It was not possible to create the output");
    let low_file = std::fs::File::create("Low Severity Results.md").expect("(Writing_to_a_file error) -> It was not possible to create the output");
    let nc_file = std::fs::File::create("Non Critical Results.md").expect("(Writing_to_a_file error) -> It was not possible to create the output");
    let files_list = [gasop_file,low_file,nc_file];

    let now = Instant::now();
    for file in &files_names{
        gasopresults = analyzing(file.to_string(), &path_name.to_string(), &gasoprules); // Analyzing every single solidity file
        lowresults = analyzing(file.to_string(), &path_name.to_string(), &lowrules);
        ncresults = analyzing(file.to_string(), &path_name.to_string(), &ncrules);
        let results_list = [gasopresults,lowresults,ncresults];

        for (index, results) in results_list.into_iter().enumerate() {
            
            if writing_to_a_file(&results, &files_list[index], file.to_string()){ // writing the results into a file
                print!("{}", "Output file created Successfully".green())
            }
            print!("\n{}{}\n","---------Results: ".green(),file.green());

            // This loop will print all the vulns/gas found by the analyzing function
            for result in results{
                if result.location.len() > 0{
                    print!("---->{}\n",result.title.bright_yellow().underline());
                    print!("     {}\n",result.description.bright_yellow());
                    for (index, bug )in result.location.into_iter().enumerate(){
                        print!("{}{}#:\n","Case ".blue(), index);
                        print!("{}\n\n", bug.bright_red().bold());
                    }
                }
            }
        }
    }

    let new_now = Instant::now();
    println!("{}{:?}","Time to complete scan: ", new_now.duration_since(now));


}


fn line_from_bytes(text: &str, start: usize, _end: usize) -> i128{
    let bytes = text.as_bytes();
    let mut count = 1;
    
    for (i, &item) in bytes.iter().enumerate() {
        // print!("{}",i);
        if i == start {
            return count;
        }
        if item == b'\n'{ 
            count = count + 1;
        }
    }
    count
}

// analyze the code using rules
// returns a struct with the results
fn analyzing(file_name: String, path_name: &String, rules: &Vec<RulesDataBase>) -> Vec<RulesDataBase>{
   
    let mut analyzed_block: Vec<RulesDataBase> = rules.to_vec();
    let foo = fs::read_to_string(format!("{}/{}", path_name, file_name)).expect("(analyzing) - Failed to open the file");

    for (indexy, rule )in rules.into_iter().enumerate(){
        let re = Regex::new(rule.rule.as_str()).unwrap();
        let mut matches = re.find_iter(&foo);
        while let Some(value) = matches.next() {
            let m = value.unwrap();
            let linenumber = line_from_bytes(&foo, m.start(), m.end());

            //Removing comment matches
            let comment_check = m.as_str().trim();
            if !comment_check.starts_with("/"){
                analyzed_block[indexy].location.push(format!("{}{}{}{}{} -> {}","(",file_name," Line-",linenumber,")", m.as_str() ));
            }
        }
        
    } 
    analyzed_block
}

// find solidity files
// returns a vector with all the solidity files found with
fn is_solidity_file(path_name: &String) -> Vec<String>{
    let mut files_names: Vec<String> = vec![];
    for entry in WalkDir::new(&path_name).into_iter().filter_map(|e| e.ok()){
    if entry.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".sol"))
                    .unwrap_or(false){
                        files_names.push(entry.file_name().to_string_lossy().to_string())
                    }
    
    }
    // println!("{}","Len:",files_names.len());
    files_names
                
}

// writes the results into a file
// returns true if everything went well
fn writing_to_a_file(results: &Vec<RulesDataBase>,mut file: &File, filename: String) -> bool{

    // file.write_all(BANNER.as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
    // file.write_all(format!("{}{}{}","\n                   Hello, Welcome to Auditoor 😀","\n\n---> Made by Shaurya Veer Singh").expect("(Writing_to_a_file error) -> It was not possible to creat the output");
    let analyzed_block: Vec<RulesDataBase> = results.to_vec();
    let mut relevant = false;
    for result in &analyzed_block{
        if result.location.len() > 0{
            relevant = true;
        }
    }
    
    if relevant {
        file.write_all(format!("# {}\n",filename).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
        let mut count = 1;
        for result in analyzed_block{
            if result.location.len() > 0{
                file.write_all(format!("## {}. {}\n",count,result.title).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                file.write_all(format!("#### {}\n",result.description).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                for (index, bug )in result.location.into_iter().enumerate(){
                    file.write_all(format!("{}{}#:\n","Case ", index).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                    file.write_all(format!("```solidity\n{}\n```\n\n", bug).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                }
                file.write_all(format!("#### {}\n",result.recommendation).as_bytes()).expect("(Writing_to_a_file error) -> Write Failed");
                count += 1;
            }
            
        }
    }

    true
}

// prints the banner
// no return
fn banner(){
    print!("{}",BANNER.bright_blue());
    println!("{}{}{}","\n                   Welcome to Auditooor 😀".blue(),"\n\n---> Made by Shaurya Veer Singh".yellow(),"\nsite: https://svskaushik.vercel.app.tech\nemail: shaurya9702@gmail.com\n\n".yellow());
    print!("{}","---------Starting----------\n\n"); 
}

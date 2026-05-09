use clap::{Parser, Subcommand};
use chrono::{Local};
use uuid::Uuid;
use core::panic;
use std::fs::File;
use std::fs;
use std::io::{ Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(version, about)]
struct ExpenseTracker {
    #[command(subcommand)]
    command: Commands
}

#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    category: String,
    amount: f64,
    date: String,
    id: String
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        category:String,

        #[arg(short, long)]
        amount: f64
    },
    Delete{
        #[arg(short, long)]
        id: String
    },
    List 
}

fn add_expense(file_name: String, expenses: Vec<Expense>) {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Unable to open {} in write mode: {}", display, why),
        Ok(file) => file 
    };

    let serialized_data = serde_json::to_string_pretty(&expenses).unwrap();

    match file.write_all(serialized_data.as_bytes())  {
        Err(why ) => panic!("Unable to add expense to {}: {}", display, why),
        Ok(_) => println!("Expenses saved successfully...")
    }
}

fn get_all_expense() -> Vec<Expense> {
    let path = Path::new("Expenses.json");
    if !path.exists() {
        return Vec::new();
    }
    let data = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(why) => panic!("Unable to read data from {}: {}", path.display(), why),
    };

    let deserialized_data = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());

    deserialized_data
}

fn main() {
    let cli = ExpenseTracker::parse();

    let mut expenses = get_all_expense();

    match &cli.command {
        Commands::Add { category, amount } => {
            println!("Adding Expense - Category: {}, Amount: {}", category, amount);

            let todays_date = Local::now().format("%Y-%m-%d").to_string();

            let expense = Expense {
                category: category.to_string(),
                amount: *amount,
                date: todays_date,
                id: Uuid::new_v4().to_string()
            };      

            expenses.push(expense);      

            add_expense("Expenses.json".to_string(), expenses);
        }
        Commands::Delete { id } => {
            println!("Deleting expense id: {}", id);

            let expenses_iter = expenses.into_iter();
            let new_expenses = expenses_iter.filter(|expense| expense.id != *id).collect();

            add_expense("Expenses.json".to_string(), new_expenses);
        }  
        Commands::List => {            
            if expenses.is_empty() {
                println!("No expenses...")
            } else {
                println!("Expenses...");

                for expense in expenses {
                    println!("Id: {}, Category: {}, Amount: {}, Date: {}", expense.id, expense.category, expense.amount, expense.date);
                }
            }
        }
    }
}

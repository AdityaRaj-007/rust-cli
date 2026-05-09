use clap::{Parser, Subcommand};
use chrono::{Local};
use uuid::Uuid;
use std::fs::File;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about)]
struct ExpenseTracker {
    #[command(subcommand)]
    command: Commands
}

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

fn main() {
    let cli = ExpenseTracker::parse();

    let mut expenses: Vec<Expense> = Vec::new();

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
        }
        Commands::Delete { id } => {
            println!("Deleting expense id: {}", id)
        }  
        Commands::List => {
            println!("Lists all the expenses...");
            
            let expense_iter = expenses.iter();

            for expense in expense_iter {
                println!("Id: {}, Category: {}, Amount: {}, Date: {}", expense.id, expense.category, expense.amount, expense.date);
            }
        }
    }
}

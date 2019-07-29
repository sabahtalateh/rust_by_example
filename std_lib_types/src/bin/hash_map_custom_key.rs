use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon..");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Logon success");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Logon failed"),
    }
}

fn main() {
    let mut accounts: Accounts = HashMap::new();

    let acc1 = Account {
        username: "ivan",
        password: "pass",
    };

    let info1 = AccountInfo {
        name: "Ivan GOG",
        email: "ivan@gog.com",
    };

    accounts.insert(acc1, info1);

    try_logon(&accounts, "ivan", "pass");
    try_logon(&accounts, "ivam", "pass");
}

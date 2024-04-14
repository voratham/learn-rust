use authentication::{login, read_line, LoginAction, LoginRole};

fn main() {
    println!("Start program Login");
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password: String = read_line();

        match login(&username, &password) {
            // LoginAction::Granted(LoginRole::Admin) => println!("Admin"),
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Admin"),
                    LoginRole::User => println!("User"),
                }
                println!("🟢 Welcome to system");
                break;
            }
            Some(LoginAction::Denied) => {}
            None => {
                println!("👌 New user system");
            }
        }

        println!("🔴 Incorrect username or password");
        tries += 1;
        if tries >= 3 {
            println!("🔥 Too many failed logins");
            break;
        }

        // if login(&username, &password) {
        //     println!("🟢 Welcome to system");
        //     break;
        // } else {
        //     println!("🔴 Incorrect username or password");
        //     tries += 1;
        // }

        // if tries >= 3 {
        //     println!("🔴 Too many failed logins");
        //     break;
        // }
    }
}

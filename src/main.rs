mod config;
mod email;
mod user;

use dotenv::dotenv;
use email::Email;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = config::Config::init();
    let user = user::User {
        name: String::from("Albert"),
        email: config.smtp_to.to_owned(),
    };

    let verification_code = "my_ultra_secure_verification_code";
    let verification_url = format!("http://localhost:3000/verifyemail/{}", verification_code);
    let email = Email::new(user, verification_url, config);
    
    if let Err(err) = email.send_verification_code().await {
        eprintln!("Failed to send verification code email: {:?}", err);
    } else {
        println!("✅Email verification code sent successfully!");
    }

    if let Err(err) = email.send_password_reset_token().await {
        eprintln!("Failed to send password reset token email: {:?}", err);
    } else {
        println!("✅Password reset token email sent successfully!");
    }

}

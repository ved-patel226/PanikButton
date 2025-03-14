use lettre::{
    transport::smtp::authentication::Credentials,
    transport::smtp::client::{ Tls, TlsParameters },
    Message,
    SmtpTransport,
    Transport,
};
use std::env;

pub fn send_email(target_email: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Build the email

    let email = Message::builder()
        .from("PanikButton <panikbuttonypar@gmail.com>".parse()?)
        .to(format!("User <{}>", target_email).parse()?)
        .subject("Test email from Rust")
        .body(String::from("This is a test email sent from Rust using Gmail SMTP!"))?;

    // Gmail SMTP credentials setup
    let gmail_address = "panikbuttonypar@gmail.com";
    let app_password = env::var("APP_PASS")?;
    let creds = Credentials::new(gmail_address.to_string(), app_password.to_string());

    // Explicitly set up TLS parameters
    let tls_parameters = TlsParameters::new("smtp.gmail.com".to_string())?;

    // Configure Gmail SMTP with explicit TLS settings
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .port(587)
        .credentials(creds)
        .tls(Tls::Required(tls_parameters))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully via Gmail!"),
        Err(e) => eprintln!("Could not send email: {e:?}"),
    }

    Ok(())
}

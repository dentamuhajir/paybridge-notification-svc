/// Generates the HTML body for a registration success email.
///
/// Takes user-specific values as parameters and injects them into
/// the template using Rust's `format!` macro — similar to string
/// interpolation in other languages.
pub fn registration_success(name: &str, verification_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Welcome to Paybridge</title>
  <style>
    body {{
      font-family: Arial, sans-serif;
      background-color: #f4f4f4;
      margin: 0;
      padding: 0;
    }}
    .container {{
      max-width: 600px;
      margin: 40px auto;
      background-color: #ffffff;
      border-radius: 8px;
      padding: 40px;
      box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }}
    .header {{
      text-align: center;
      margin-bottom: 32px;
    }}
    .header h1 {{
      color: #1a1a2e;
      font-size: 24px;
    }}
    .body-text {{
      color: #444444;
      font-size: 15px;
      line-height: 1.6;
    }}
    .cta-button {{
      display: inline-block;
      margin-top: 24px;
      padding: 12px 28px;
      background-color: #4f46e5;
      color: #ffffff !important;
      text-decoration: none;
      border-radius: 6px;
      font-size: 15px;
      font-weight: bold;
    }}
    .footer {{
      margin-top: 40px;
      font-size: 12px;
      color: #999999;
      text-align: center;
    }}
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <h1>Welcome to Paybridge 🎉</h1>
    </div>
    <p class="body-text">Hi <strong>{name}</strong>,</p>
    <p class="body-text">
      Thank you for registering! Your account has been created successfully.
      Please verify your email address to get started.
    </p>
    <a href="{verification_link}" class="cta-button">Verify My Email</a>
    <p class="body-text" style="margin-top: 24px;">
      If you did not create this account, you can safely ignore this email.
    </p>
    <div class="footer">
      &copy; 2025 Paybridge. All rights reserved.<br/>
      This is an automated message, please do not reply.
    </div>
  </div>
</body>
</html>
        "#,
        // `{{` and `}}` in the format string are escaped braces (literal `{` `}` in output).
        // `{name}` and `{verification_link}` are the actual injected values.
        name = name,
        verification_link = verification_link,
    )
}
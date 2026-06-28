For a Rust web application like KelpieBooks, I would recommend a combination of:

1. **A configuration file (TOML)**
2. **Environment variable overrides for secrets and deployment-specific values**

This is the approach used by many mature Rust applications.

### Why not environment variables only?

Environment variables work well for:

* Database passwords
* API keys
* SMTP credentials
* OAuth secrets

But they become painful for:

* Feature flags
* UI settings
* Accounting options
* Localization settings
* Email templates
* Logging configuration

Once you have more than about 10 settings, a configuration file is much easier to manage.

### Why TOML?

Since you're already in the Rust ecosystem:

* Cargo uses TOML
* Rocket uses TOML
* Many Rust crates support TOML directly
* Easy to read and edit manually
* Comments are supported

Example:

```toml
[server]
host = "0.0.0.0"
port = 8000
public_url = "https://kelpiebooks.org"

[database]
max_connections = 20

[email]
smtp_server = "smtp.example.com"
smtp_port = 587
from_address = "noreply@kelpiebooks.org"

[i18n]
default_locale = "en-AU"
```

### Recommended project layout

```text
kelpiebooks/
├── config/
│   ├── default.toml
│   ├── development.toml
│   └── production.toml
├── backend/
├── frontend/
└── shared-core/
```

For example:

**default.toml**

```toml
[server]
port = 8000

[i18n]
default_locale = "en-AU"
```

**production.toml**

```toml
[server]
public_url = "https://kelpiebooks.org"
```

### Environment variables for secrets

Keep secrets out of Git:

```bash
DATABASE_URL=postgres://...
SMTP_PASSWORD=...
JWT_SECRET=...
```

Then merge them into the loaded configuration.

### Popular Rust crates

A common combination is:

```toml
config = "0.15"
serde = { version = "1", features = ["derive"] }
```

Configuration struct:

```rust
#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub server: Server,
    pub email: Email,
}

#[derive(Debug, serde::Deserialize)]
pub struct Server {
    pub public_url: String,
    pub port: u16,
}
```

Load:

```rust
use config::{Config, Environment, File};

let settings: Settings = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name("config/production").required(false))
    .add_source(Environment::with_prefix("KELPIE"))
    .build()?
    .try_deserialize()?;
```

Then:

```rust
settings.server.public_url
```

### For KelpieBooks

Given that you're building a full accounting system, I would start with:

```toml
[server]
public_url = "http://localhost:8000"

[database]
max_connections = 20

[email]
enabled = false

[i18n]
default_locale = "en-AU"

[company]
default_country = "AU"
default_currency = "AUD"
```

and store only secrets such as SMTP passwords, JWT secrets, and database credentials in environment variables.

That gives you a configuration system that will scale well as the application grows from a few modules to a complete SME accounting package.

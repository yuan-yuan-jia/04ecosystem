use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;
use anyhow::Result;

#[derive(Builder, Debug)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: String,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(setter(each(name = "skill", into)), default = "vec![]")]
    skills: Vec<String>,
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }

    pub fn dob(&mut self, value: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();

        self
    }
}

fn main() -> Result<()> {
    let user = User::builder()
        .name("Alice")
        .skill("programming")
        .skill("debugging")
        .email("try@awesome.com")
        .dob("1990-01-01T00:00:00Z")
        .build()?;

    println!("{:#?}", user);

    Ok(())
}
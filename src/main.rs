use colored::*;

fn main() {
    println!(
        r#"
{header}

  {about}

{i_am}

{b} 🚧 working on {working_on}
{b} ✨ a {rust} fanboy
{b} 🐦 tweeting on Twitter as {twitter}
{b} 💡 collaborating on GitHub as {github} 
{b} 🇩🇪 located in {location}
{b} 🔗 blogging at {url}

"#,
        b = " »".dimmed(),
        header = "»»»» About me ««««".bold(),
        about = "Working on open source, cloud scale IoT messaging".italic(),
        i_am = "›››› I am ‹‹‹‹".bold(),
        working_on = "IoT stuff".bold(),
        rust = "Rust".bold(),
        twitter = "@ctron".bold(),
        github = "@ctron".bold(),
        location = "Germany".bold(),
        url = "https://dentrassi.de".underline(),
    );
}

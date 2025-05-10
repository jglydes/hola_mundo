use anchor_lang::prelude::*;

declare_id!("Eg9n2jQm57XDTyhESDzbXp96CunY5SFirmNovq49UBxr");

#[program]
pub mod hola_mundo {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hola, Mundo from Cavite!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "SolBetX",
    project_url: "https://solbetx.com/",
    contacts: "mailto:office@vtscc.org,https://vtscc.org/contact.html",
    policy: "https://solbetx.com/security-policy",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/Vermont-Secure-Computing/bitbet",
    source_revision: "Eg9n2jQm57XDTyhESDzbXp96CunY5SFirmNovq49UBxr",
    source_release: "",
    encryption: "",
    auditors: "vtscc.org",
    acknowledgements: "Bet big - Win big!"
}

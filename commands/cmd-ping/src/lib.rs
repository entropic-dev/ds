use std::io::{self, Write};
use std::time::Instant;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use serde::Deserialize;
use serde_json::Value;
use structopt::StructOpt;
use surf;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct PingCmd {
    #[structopt(
        help = "Registry to ping.",
        default_value = "https://registry.entropic.dev"
    )]
    registry: Url,
    #[structopt(long, help = "Format output as JSON.")]
    json: bool,
}

#[derive(Deserialize)]
struct EntropicError {
    message: String,
}

#[async_trait]
impl DsCommand for PingCmd {
    async fn execute(mut self, args: ArgMatches<'_>, config: Config) -> Result<()> {
        if args.occurrences_of("registry") == 0 {
            self.registry = Url::parse(
                &config
                    .get_str("registry")
                    .unwrap_or("https://registry.entropic.dev".into()),
            )
            .context("Failed to parse registry URL")?;
        }
        self.ping(io::stdout(), io::stderr()).await
    }
}

impl PingCmd {
    async fn ping<O, E>(self, mut stdout: O, mut stderr: E) -> Result<()>
    where
        O: Write,
        E: Write,
    {
        writeln!(stderr, "PING: {}", self.registry)?;
        let start = Instant::now();
        // This silliness is due to silliness in Surf that should be addressed
        // soon. Once it's fixed, this line will just be a nice .await? See:
        // https://github.com/dtolnay/anyhow/issues/35#issuecomment-547986739
        let mut res = match surf::get(&self.registry).await {
            Ok(response) => response,
            Err(err) => bail!(err),
        };
        if res.status().as_u16() >= 400 {
            let msg = match res.body_json::<EntropicError>().await {
                Ok(err) => err.message,
                Err(_) => match res.body_string().await {
                    Ok(msg) => msg,
                    Err(err) => bail!("Failed to get response body: {}", err),
                },
            };
            return Err(anyhow!("Ping failed for {}: {}", self.registry, msg));
        }

        let time = start.elapsed().as_millis() as u64;
        writeln!(stderr, "PONG: {}ms", time)?;
        if self.json {
            let details: Value =
                serde_json::from_str(&res.body_string().await.unwrap_or("{}".into()))
                    .context("Failed to parse response body")?;
            writeln!(
                stdout,
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "registry": self.registry.to_string(),
                    "time": time,
                    "details": details,
                }))?
            )?;
        } else {
            writeln!(
                stderr,
                "PONG: {}",
                res.body_string().await.unwrap_or("".into())
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use async_std;
    use mockito::mock;
    use serde_json::json;

    #[async_std::test]
    async fn basic() -> Result<()> {
        let m = mock("GET", "/")
            .with_status(200)
            .with_body("hello, world!")
            .create();
        let registry = &mockito::server_url();
        let mut stdout: Vec<u8> = Vec::new();
        let mut stderr: Vec<u8> = Vec::new();
        let cmd = PingCmd {
            registry: Url::parse(registry)?,
            json: false,
        };
        cmd.ping(&mut stdout, &mut stderr).await?;
        m.assert();
        assert_eq!(String::from_utf8(stdout)?, "");
        let stderr = String::from_utf8(stderr)?;
        assert!(stderr.contains(&format!("PING: {}", registry)));
        assert!(stderr.contains("PONG:"));
        assert!(stderr.contains("hello, world!"));
        Ok(())
    }

    #[async_std::test]
    async fn json() -> Result<()> {
        let m = mock("GET", "/")
            .with_status(200)
            .with_body(r#"{"message": "hello, world!"}"#)
            .create();
        let registry = &mockito::server_url();
        let mut stdout: Vec<u8> = Vec::new();
        let mut stderr: Vec<u8> = Vec::new();
        let cmd = PingCmd {
            registry: Url::parse(registry)?,
            json: true,
        };

        cmd.ping(&mut stdout, &mut stderr).await?;
        m.assert();

        let stdout = String::from_utf8(stdout)?;
        assert!(stdout.contains(r#""message": "hello, world!""#));
        let mut parsed = serde_json::from_str::<Value>(&stdout)?;
        assert!(parsed["time"].take().is_number());
        assert_eq!(
            parsed,
            json!({
                "registry": Url::parse(registry)?.to_string(),
                "details": {
                    "message": "hello, world!"
                },
                "time": null,
            })
        );

        let stderr = String::from_utf8(stderr).unwrap();
        assert!(stderr.contains(&format!("PING: {}", registry)));
        assert!(stderr.contains("PONG:"));

        Ok(())
    }
}

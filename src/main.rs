use async_std::task;
use cargo_toml::{Manifest, Value};
use std::{io::Read, path::PathBuf};
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
pub struct Install {}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Install(Install),
}

#[derive(Debug, Error)]
enum Error {
    #[error("No Cargo.toml could be found")]
    NoCargoToml,
    #[error("There was an error downloading the NuGet package {0}")]
    DownloadError(Box<dyn std::error::Error>),
    #[error("The Cargo.toml file was malformed")]
    MalformedManifest,

    #[error("Other errors")]
    Other(Box<dyn std::error::Error>),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "nuget")]
struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Install(i) => i.perform().unwrap(),
    }
}

impl Install {
    fn perform(&self) -> Result<(), Error> {
        let bytes = std::fs::read("Cargo.toml").map_err(|_| Error::NoCargoToml)?;
        let manifest: Manifest<Value> =
            Manifest::from_slice(&bytes).map_err(|_| Error::MalformedManifest)?;
        let deps = get_deps(manifest)?;
        let downloaded_deps = download_dependencies(deps)?;

        for (dep, zipped_bytes) in downloaded_deps {
            let reader = std::io::Cursor::new(zipped_bytes);
            let mut zip = zip::ZipArchive::new(reader).map_err(|e| Error::Other(Box::new(e)))?;
            let mut winmds = Vec::new();
            for i in 0..zip.len() {
                let mut file = zip.by_index(i).unwrap();
                let path = file.mangled_name();
                match path.extension() {
                    Some(e) if e == "winmd" => {
                        let name = path.file_name().unwrap().to_owned();
                        let mut contents = Vec::with_capacity(file.size() as usize);

                        if let Err(e) = std::io::copy(&mut file, &mut contents) {
                            println!("Could not read file: {:?}, {:?}",e , file.compression());
                        }

                        winmds.push((name, contents));
                    }
                    _ => {}
                }
            }

            let dep_directory = PathBuf::new().join("target").join("nuget").join(dep.name);
            std::fs::create_dir_all(&dep_directory).unwrap();
            for (name, contents) in winmds {
                std::fs::write(&dep_directory.join(name), contents).unwrap();
            }
        }

        Ok(())
    }
}

fn get_deps(manifest: Manifest) -> Result<Vec<Dependency>, Error> {
    let metadata = manifest.package.and_then(|p| p.metadata);

    match metadata {
        Some(Value::Table(mut t)) => {
            let deps = match t.remove("nuget_dependencies") {
                Some(Value::Table(deps)) => deps,
                _ => return Err(Error::MalformedManifest.into()),
            };
            deps.into_iter()
                .map(|(key, value)| match value {
                    Value::String(version) => Ok(Dependency::new(key, version)),
                    _ => Err(Error::MalformedManifest.into()),
                })
                .collect()
        }
        _ => return Err(Error::MalformedManifest.into()),
    }
}

#[derive(Debug)]

struct Dependency {
    name: String,
    version: String,
}

impl Dependency {
    fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    fn url(&self) -> String {
        format!(
            "https://www.nuget.org/api/v2/package/{}/{}",
            self.name, self.version
        )
    }

    async fn download(&self) -> Result<Vec<u8>, Error> {
        let mut res = surf::get(self.url())
            .await
            .map_err(|e| Error::DownloadError(e))?;
        match res.status().into() {
            200u16 => {}
            302 => {
                let headers = res.headers();
                let redirect_utl = headers.get("Location").unwrap();
                res = surf::get(redirect_utl).await.unwrap();
                assert_eq!(res.status(), 200);
            }
            _ => {
                return Err(Error::DownloadError(
                    anyhow::anyhow!("Not 200 response: {}", res.status()).into(),
                ))
            }
        };
        let bytes = res
            .body_bytes()
            .await
            .map_err(|e| Error::DownloadError(e.into()))?;
        Ok(bytes)
    }
}

fn download_dependencies(deps: Vec<Dependency>) -> Result<Vec<(Dependency, Vec<u8>)>, Error> {
    task::block_on(async {
        let results = deps.into_iter().map(|dep| async move {
            let bytes = dep.download().await?;
            Ok((dep, bytes))
        });

        futures::future::try_join_all(results).await
    })
}

use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashSet, env};
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@brave/brave-search-mcp-server";

struct BraveSearchModelContextExtension {
    installed: HashSet<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct BraveSearchContextServerSettings {
    brave_api_key: String,
}

impl BraveSearchModelContextExtension {
    fn install_package_if_needed(&mut self, package_name: &str) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(package_name)?;

        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        let latest_version = zed::npm_package_latest_version(package_name)?;
        if installed_version.as_ref() != Some(&latest_version) {
            println!("Installing {package_name}@{latest_version}...");
            if let Err(error) = zed::npm_install_package(package_name, &latest_version) {
                if installed_version.is_none() {
                    Err(error)?;
                }
            }
        } else {
            println!("Found {package_name}@{latest_version} installed");
        }

        self.installed.insert(package_name.to_string());
        Ok(())
    }
}

impl zed::Extension for BraveSearchModelContextExtension {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
        }
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        self.install_package_if_needed(PACKAGE_NAME)?;

        let settings = ContextServerSettings::for_project("mcp-server-brave-search", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `brave_api_key` setting".into());
        };
        let settings: BraveSearchContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                zed_ext::sanitize_windows_path(env::current_dir().unwrap())
                    .join("node_modules")
                    .join(PACKAGE_NAME)
                    .join("dist/index.js")
                    .to_string_lossy()
                    .to_string(),
                "--transport".into(),
                "stdio".into(),
            ],
            env: vec![("BRAVE_API_KEY".into(), settings.brave_api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(BraveSearchContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(BraveSearchModelContextExtension);

mod zed_ext {
    /// Workaround for https://github.com/bytecodealliance/wasmtime/issues/10415.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{current_platform, Os};
        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .to_string()
                .trim_start_matches('/')
                .into(),
        }
    }
}

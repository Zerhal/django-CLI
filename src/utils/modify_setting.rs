use crate::utils::errors::ProjectError;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SettingsModification {
    AddToInstalledApps(String),
    AddOrUpdateVariable(String, String),
    AddOrUpdateDict(String, Vec<(String, String)>),
}

pub fn modify_settings(
    project_name: &str,
    modifications: &[SettingsModification],
) -> Result<(), ProjectError> {
    let settings_path = PathBuf::from(project_name).join("settings.py");
    println!("settings_path: {:?}", settings_path);
    println!("modifications: {:?}", modifications);
    let content = std::fs::read_to_string(&settings_path).map_err(ProjectError::Io)?;

    let mut modifier = SettingsModifier::new(content);

    for modification in modifications {
        match modification {
            SettingsModification::AddToInstalledApps(app_name) => {
                modifier.add_to_installed_apps(app_name);
            }
            SettingsModification::AddOrUpdateVariable(name, value) => {
                modifier.add_or_update_variable(name, value)?;
            }
            SettingsModification::AddOrUpdateDict(name, entries) => {
                modifier.add_or_update_dict(name, entries)?;
            }
        }
    }

    std::fs::write(&settings_path, modifier.finalize()).map_err(ProjectError::Io)?;

    Ok(())
}

struct SettingsModifier {
    content: String,
}

impl SettingsModifier {
    fn new(content: String) -> Self {
        Self { content }
    }

    fn add_to_installed_apps(&mut self, app_name: &str) {
        if !self.content.contains(&format!("'{}'", app_name)) {
            self.content = self.content.replace(
                "INSTALLED_APPS = [",
                &format!("INSTALLED_APPS = [\n    '{}',", app_name),
            );
            println!("Added '{}' to INSTALLED_APPS.", app_name);
        }
    }

    fn add_or_update_variable(&mut self, name: &str, value: &str) -> Result<(), ProjectError> {
        let re = Regex::new(&format!(r"(?m)^{}\s*=.*$", regex::escape(name)))
            .map_err(|e| ProjectError::CommandFailed(format!("Regex error: {}", e)))?;

        let formatted_value = self.format_value(value);
        if re.is_match(&self.content) {
            self.content = re
                .replace(&self.content, &format!("{} = {}", name, formatted_value))
                .to_string();
            println!("Updated variable {} in settings.py.", name);
        } else {
            self.content
                .push_str(&format!("\n{} = {}\n", name, formatted_value));
            println!("Added variable {} to settings.py.", name);
        }
        Ok(())
    }

    fn add_or_update_dict(
        &mut self,
        name: &str,
        entries: &[(String, String)],
    ) -> Result<(), ProjectError> {
        println!("Starting add_or_update_dict for {}", name);

        let mut dict_entries: HashMap<String, String> = HashMap::new();
        let mut found_dict = false;
        let mut updated_content = String::new();

        for line in self.content.lines() {
            if line.trim_start().starts_with(&format!("{} =", name)) {
                found_dict = true;
                updated_content.push_str(&format!("{} = {{\n", name));

                // Continue to process the lines within the dictionary
                for dict_line in self
                    .content
                    .lines()
                    .skip_while(|l| !l.trim_start().starts_with(&format!("{} =", name)))
                    .skip(1)
                {
                    if dict_line.trim_start().starts_with("}") {
                        break;
                    }
                    if let Some((key, value)) = self.parse_dict_entry(dict_line) {
                        dict_entries.insert(key, value);
                    }
                }

                // Add new or updated entries
                for (key, value) in entries {
                    dict_entries.insert(key.clone(), value.clone());
                    println!("Updated/Added entry for key '{}'", key);
                }

                // Write the updated dictionary back
                for (k, v) in &dict_entries {
                    updated_content.push_str(&format!("    '{}': {},\n", k, self.format_value(v)));
                }
                updated_content.push_str("}\n");
            } else if !found_dict {
                // Before the dictionary, keep the original line
                updated_content.push_str(line);
                updated_content.push('\n');
            } else {
                // Skip the old dictionary content
                if line.trim_start().starts_with('}') {
                    found_dict = false;
                }
            }
        }

        // If the dictionary wasn't found, append it at the end
        if !found_dict {
            updated_content.push_str(&format!("\n{} = {{\n", name));
            for (key, value) in entries {
                updated_content.push_str(&format!(
                    "    '{}': {},\n",
                    key,
                    self.format_value(value)
                ));
            }
            updated_content.push_str("}\n");
        }

        self.content = updated_content;
        println!("Final dictionary for {}:\n{}", name, self.content);

        Ok(())
    }

    fn parse_dict_entry(&self, line: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            Some((
                parts[0].trim().trim_matches('\'').to_string(),
                parts[1].trim().trim_matches(',').to_string(),
            ))
        } else {
            None
        }
    }

    fn format_value(&self, value: &str) -> String {
        if value.starts_with('[') && value.ends_with(']') {
            // Si la valeur est déjà une liste ou une chaîne correctement formatée, ne pas ajouter de guillemets
            value.to_string()
        } else if value.starts_with('\'') && value.ends_with('\'') {
            // Si la valeur est déjà entre guillemets, ne pas la doubler
            value.to_string()
        } else {
            // Sinon, ajouter des guillemets
            format!("'{}'", value)
        }
    }

    fn finalize(self) -> String {
        self.content
    }
}

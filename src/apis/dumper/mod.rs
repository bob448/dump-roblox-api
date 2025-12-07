use serde::Deserialize;

use class::Class;
use enums::Enum;

pub async fn try_dump(studio_version: String) -> Result<ApiDump, reqwest::Error> {
    let url = API_DUMP_URL.replace("{v}", &studio_version);

    let response = reqwest::get(url).await?;
    response.json().await
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ApiDump {
    pub Classes: Vec<Class>,
    pub Enums: Vec<Enum>,
    pub Version: u32,
}

impl ApiDump {
    pub fn class_names_into_luau_table(
        &self,
        hide_non_creatable: bool,
        hide_non_browseable: bool
    ) -> String {
        let mut buf = "{\n".to_string();

        let mut first = true;

        for (i, class) in self.Classes.iter().enumerate() {
            if hide_non_browseable && class.is_not_browsable() {
                continue;
            } else if hide_non_creatable && class.is_not_createable() {
                continue;
            }

            if !first {
                buf.push(',');
                buf.push('\n');
            }

            if first {
                first = false;
            }

            buf.push_str(&format!("\t\"{}\"", class.Name));
        }

        buf.push('\n');

        buf.push('}');

        buf
    }
    pub fn classes_into_luau_table(
        &self,
        hide_non_creatable: bool,
        hide_non_browseable: bool
    ) -> String {
        let mut buf = "{\n".to_string();
        let mut first = true;

        for (i, class) in self.Classes.iter().enumerate() {
            if
                (hide_non_browseable && class.is_not_browsable()) ||
                (hide_non_creatable && class.is_not_createable())
            {
                continue;
            }

            if !first {
                buf.push(',');
                buf.push('\n');
            }

            if first {
                first = false;
            }

            buf.push_str(&format!("\t[\"{}\"] = {{\n", class.Name));
            buf.push_str("\t\t[\"Members\"] = {\n");

            for (ii, member) in class.Members.iter().enumerate() {
                if ii != 0 {
                    buf.push(',');
                    buf.push('\n');
                }

                buf.push_str(
                    &format!(
                        "\t\t\t{{\n\
                        \t\t\t\t[\"Category\"] = \"{}\",\n\
                        \t\t\t\t[\"MemberType\"] = \"{}\",\n\
                        \t\t\t\t[\"Name\"] = \"{}\",\n\
                        \t\t\t\t[\"ValueType\"] = {{\n\
                        \t\t\t\t\t[\"Name\"] = \"{}\",\n\
                        \t\t\t\t\t[\"Category\"] = \"{}\"\n\
                        \t\t\t\t}}\n\
                        \t\t\t}}",
                        member.Category,
                        member.MemberType,
                        member.Name.replace("\"", "\\\""),
                        member.ValueType.Name,
                        member.ValueType.Category
                    )
                );
            }

            buf.push('\n');

            buf.push_str("\t\t},\n");

            buf.push_str(
                &format!(
                    "\t\t[\"MemoryCategory\"] = \"{}\",\n\
                    \t\t[\"Superclass\"] = \"{}\",\n\
                    \t\t[\"Tags\"] = {{\n",
                    class.MemoryCategory,
                    class.Superclass
                )
            );

            for (ii, tag) in class.Tags.iter().enumerate() {
                if ii != 0 {
                    buf.push(',');
                    buf.push('\n');
                }

                buf.push_str(&format!("\t\t\t\"{}\"", tag));
            }

            buf.push('\n');

            buf.push_str("\t\t}\n");

            buf.push_str(&format!("\t}}"));
        }

        buf.push('\n');
        buf.push('}');

        buf
    }
    pub fn enums_into_luau_table(&self) -> String {
        let mut buf = "{\n".to_string();

        for (i, e) in self.Enums.iter().enumerate() {
            if i != 0 {
                buf.push(',');
                buf.push('\n');
            }

            buf.push_str(&format!("\t[\"{}\"] = {{\n", e.Name));

            for (ii, enum_item) in e.Items.iter().enumerate() {
                if ii != 0 {
                    buf.push(',');
                    buf.push('\n');
                }

                buf.push_str(&format!("\t\t[{}] = \"{}\"", enum_item.Value, enum_item.Name));
            }

            buf.push('\n');

            buf.push_str(&format!("\t}}"));
        }

        buf.push('\n');

        buf.push('}');

        buf
    }
}

const API_DUMP_URL: &str = "https://s3.amazonaws.com/setup.roblox.com/{v}-API-Dump.json";

pub mod class;
pub mod enums;

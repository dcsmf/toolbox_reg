use std::io::ErrorKind;
use std::{fs, path::Path};

use crate::config::ToolsDto;
use crate::reg_util::{self, RegValue};
use winreg::enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER};

/**
 * 检查toolbox是否存在
 */
#[tauri::command]
pub fn check_toolbox_exist(toolbox_path: &str) -> bool {
    if toolbox_path.is_empty() {
        return false;
    }
    let toolbox_name = "jetbrains-toolbox.exe";
    let file_path = format!("{}\\{}", toolbox_path, toolbox_name);
    fs::metadata(file_path).is_ok()
}

/**
 * 从注册表读取toolbox安装路径
 */
#[tauri::command]
pub fn from_reg_get_path() -> Result<String, String> {
    reg_util::get_reg_value(HKEY_CURRENT_USER, r"Software\JetBrains\Toolbox", "")
        .map_err(|e| e.to_string())
}

/**
 * 取toolbox上级目录的某个文件内容为字符串
 */
#[tauri::command]
pub fn get_state_file_str(toolbox_path: &str, file_name: &str) -> String {
    let toolbox_path = Path::new(toolbox_path);
    let parent_dir = toolbox_path.parent();
    let file_path = parent_dir.map(|parent| parent.join(file_name));
    let file_str = file_path.and_then(|path| fs::read_to_string(path).ok());
    file_str.unwrap_or_default()
}

/**
 * 设置右键菜单注册表
 */
#[tauri::command]
pub fn set_ide_info_to_reg(tools: Vec<ToolsDto>, toolbox_path: &str) -> Result<(), String> {
    use std::collections::HashMap;

    let h_key = HKEY_CLASSES_ROOT;
    let mut values = Vec::new();

    // 定义顶层参数
    let top_values = HashMap::from([
        ("Icon".to_owned(), toolbox_path.to_owned() + r"\toolbox.ico"),
        ("SubCommands".to_owned(), String::new()),
        ("MUIVerb".to_owned(), "Open Toolbox Here".to_owned()),
    ]);

    // 定义shell层和里面的command层参数
    let default_shell_values = HashMap::from([
        (String::new(), String::new()),
        ("Icon".to_owned(), toolbox_path.to_owned()),
    ]);
    let default_command_values = HashMap::from([(String::new(), String::new())]); // Empty string placeholder

    // 设置顶层的值
    values.push(RegValue {
        sub_key: String::from(r"Directory\shell\jb-toolbox"),
        values: top_values.clone(),
    });
    values.push(RegValue {
        sub_key: String::from(r"Directory\Background\shell\jb-toolbox"),
        values: top_values.clone(),
    });

    // 遍历工具，并赋值
    for tool in tools {
        // 覆盖默认值
        let shell_values = {
            let mut values = default_shell_values.clone();
            values.insert(String::new(), format!("Open {} Here", tool.name));
            values.insert("Icon".to_owned(), tool.location.to_owned());
            values
        };
        let command_values = {
            let mut values = default_command_values.clone();
            values.insert(String::new(), format!(r##""{}" "%V""##, tool.location));
            values
        };

        // 构建对象
        for location in &[
            r"Directory\shell\jb-toolbox\shell",
            r"Directory\Background\shell\jb-toolbox\shell",
        ] {
            values.push(RegValue {
                sub_key: format!(r"{}\{}", location, tool.tag),
                values: shell_values.clone(),
            });
            values.push(RegValue {
                sub_key: format!(r"{}\{}\command", location, tool.tag),
                values: command_values.clone(),
            });
        }
    }

    reg_util::set_values(h_key, values).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_ide_info_from_reg() -> Result<(), String> {
    reg_util::delete_sub_key(HKEY_CLASSES_ROOT, r"Directory\shell\jb-toolbox")
        .or_else(|e| {
            if e.kind() == ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })
        .map_err(|e| e.to_string())?;
    reg_util::delete_sub_key(HKEY_CLASSES_ROOT, r"Directory\Background\shell\jb-toolbox")
        .or_else(|e| {
            if e.kind() == ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })
        .map_err(|e| e.to_string())
}

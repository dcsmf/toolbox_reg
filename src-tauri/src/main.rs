#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use toolbox_reg::cmds;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::check_toolbox_exist,
            cmds::from_reg_get_path,
            cmds::get_state_file_str,
            cmds::set_ide_info_to_reg,
            cmds::clear_ide_info_from_reg
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

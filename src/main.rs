use path::MAIN_SEPARATOR_STR;
use std::{env, fs, path};
use std::path::Path;
use std::process::Command;
use os_release::OsRelease;

/*
---------------------------
Modern Slavonic Installer Program
---------------------------
Purpose:
- back up system base.xml and evdev.xml xkb-rule-files
- write xkb-rules into the system base.xml and evdev.xml
- install xkb layout file into system symbols
(this program needs root permissions)
(this program can be used to update your installation,
after system updates braking the installation)
---------------------------
*/

// TODO: use local $HOME/.xkb/rules/evdev.xml instead of using system files
// print!(env!("HOME"));

fn main() {

    // determine OS and select installation process
    match env::consts::OS {
        "linux" => {
            // set string reference to local source files
            let res_str_ref_xkb_file = "res".to_owned()
                + MAIN_SEPARATOR_STR + "sla.xkb";
            let res_str_ref_xml_layout_file = "res".to_owned()
                + MAIN_SEPARATOR_STR + "sla_layout.xml";
            let res_str_ref_evdev_xml_file = "res".to_owned()
                + MAIN_SEPARATOR_STR + "evdev.xml";

            // set string reference to local backup
            let bk_str_ref_bk_folder = "bk".to_owned();
            let bk_str_ref_bk_base_xml = "bk".to_owned()
                + MAIN_SEPARATOR_STR + "base.xml";
            let bk_str_ref_bk_evdev_xml = "bk".to_owned()
                + MAIN_SEPARATOR_STR + "evdev.xml";

            // set string reference to local temp file
            let temp_str_ref_temp_xml_file = "temp.xml".to_owned();

            // set path reference to local source files
            let path_sla_xkb = Path::new(&res_str_ref_xkb_file);
            let path_sla_xml = Path::new(&res_str_ref_xml_layout_file);

            if path_sla_xkb.exists() == false {
                println!("installation corrupted, missing xkb file, aborting");
                return;
            }
            if path_sla_xml.exists() == false {
                println!("installation corrupted, missing xml file, aborting");
                return;
            }

            // set path reference to local backup
            let path_backup_folder = Path::new(&bk_str_ref_bk_folder);
            let path_backup_base_xml = Path::new(&bk_str_ref_bk_base_xml);
            let path_backup_evdev_xml = Path::new(&bk_str_ref_bk_evdev_xml);

            // set string references to system files
            let sys_str_ref_symbols_folder;
            let sys_str_ref_xkb_file;
            let sys_str_ref_base_xml_file;
            let sys_str_ref_evdev_xml_file;

            // determine linux distribution paths
            match OsRelease::new().unwrap().id_like.as_str() {
                "arch" => {
                    // set system symbols folder string reference
                    sys_str_ref_symbols_folder = MAIN_SEPARATOR_STR.to_owned()
                        + "usr" + MAIN_SEPARATOR_STR
                        + "share" + MAIN_SEPARATOR_STR
                        + "X11" + MAIN_SEPARATOR_STR
                        + "xkb" + MAIN_SEPARATOR_STR
                        + "symbols";

                    // set system base_xml file string reference
                    sys_str_ref_base_xml_file = MAIN_SEPARATOR_STR.to_owned()
                        + "usr" + MAIN_SEPARATOR_STR
                        + "share" + MAIN_SEPARATOR_STR
                        + "X11" + MAIN_SEPARATOR_STR
                        + "xkb" + MAIN_SEPARATOR_STR
                        + "rules" + MAIN_SEPARATOR_STR
                        + "base.xml";

                    // set system evdev_xml file string reference
                    sys_str_ref_evdev_xml_file = MAIN_SEPARATOR_STR.to_owned()
                        + "usr" + MAIN_SEPARATOR_STR
                        + "share" + MAIN_SEPARATOR_STR
                        + "X11" + MAIN_SEPARATOR_STR
                        + "xkb" + MAIN_SEPARATOR_STR
                        + "rules" + MAIN_SEPARATOR_STR
                        + "evdev.xml";
                }

                "debian" => {
                    println!("currently not implemented");
                    todo!("currently not implemented");
                }

                _ => {
                    println!("unknown distribution, aborting");
                    return;
                }
            }

            // set system target xkb file reference
            sys_str_ref_xkb_file = sys_str_ref_symbols_folder.to_owned() + MAIN_SEPARATOR_STR + "sla";

            // set system files paths
            let path_sys_symbols_folder = Path::new(&sys_str_ref_symbols_folder);
            let path_sys_sla_file = Path::new(&sys_str_ref_xkb_file);
            let path_sys_base_xml = Path::new(&sys_str_ref_base_xml_file);
            let path_sys_evdev_xml = Path::new(&sys_str_ref_evdev_xml_file);

            // check existence of crucial system paths
            if (path_sys_symbols_folder.exists() == false || path_sys_base_xml.exists() == false || path_sys_evdev_xml.exists() == false) {
                println!("system paths invalid, aborting");
                return;
            }

            /*
            backup process START
            */

            // check existence of local backup folder
            if (path_backup_folder.exists() == false) {
                fs::create_dir_all(path_backup_folder)
                    .expect("failed to create backup folder");
            }

            // create backup of system base.xml
            if (path_backup_base_xml.exists() == false) {
                let command_backup_base_xml = format!("sudo cp {} {}", sys_str_ref_base_xml_file, bk_str_ref_bk_base_xml);
                let output_cmd_backup_base_xml = Command::new("sh")
                    .arg("-c")
                    .arg(command_backup_base_xml)
                    .output()
                    .expect("backup of base.xml rules file failed");
                if (output_cmd_backup_base_xml.status.success() == false) {
                    let error_base_xml_copy = String::from_utf8_lossy(&output_cmd_backup_base_xml.stderr);
                    println!("Error: {}", error_base_xml_copy);
                } else {
                    println!("backup of base.xml successful");
                }
            } else {
                println!("backup of base.xml already exists");
            }

            // create backup of system evdev.xml
            if (path_backup_evdev_xml.exists() == false) {
                let command_backup_evdev_xml = format!("sudo cp {} {}", sys_str_ref_evdev_xml_file, bk_str_ref_bk_evdev_xml);
                let output_cmd_backup_evdev_xml = Command::new("sh")
                    .arg("-c")
                    .arg(command_backup_evdev_xml)
                    .output()
                    .expect("backup of evdev.xml rules file failed");
                if (output_cmd_backup_evdev_xml.status.success() == false) {
                    let error_evdev_xml_copy = String::from_utf8_lossy(&output_cmd_backup_evdev_xml.stderr);
                    println!("Error: {}", error_evdev_xml_copy);
                } else {
                    println!("backup of evdev.xml successful");
                }
            } else {
                println!("backup of evdev.xml already exists");
            }

            /*
            installation process xkb START
            */

            // check previous installation footprint
            if (path_sys_sla_file.exists()) {
                println!("detected previous installation");
                println!("please approve the remove of {}", sys_str_ref_xkb_file);
                let command_clean_xkb = format!("sudo rm -i {}", sys_str_ref_xkb_file);
                let output_command_clean_xkb = Command::new("sh")
                    .arg("-c")
                    .arg(command_clean_xkb)
                    .output()
                    .expect("failed to clean previous xkb file");
                if (output_command_clean_xkb.status.success() == false) {
                    let error_clean_xkb = String::from_utf8_lossy(&output_command_clean_xkb.stderr);
                    println!("Error: {}", error_clean_xkb);
                } else {
                    println!("cleaning of xkb-file successful");
                }
            }

            // copy local xkb file to system symbols folder
            let command_install_xkb = format!("sudo cp {} {}", res_str_ref_xkb_file, sys_str_ref_xkb_file);
            let output_command_instal_xkb = Command::new("sh")
                .arg("-c")
                .arg(command_install_xkb)
                .output()
                .expect("failed to install xkb file");
            if (output_command_instal_xkb.status.success() == false) {
                let error_xkb_installation = String::from_utf8_lossy(&output_command_instal_xkb.stderr);
                println!("Error: {}", error_xkb_installation);
            } else {
                println!("installation of xkb-file successful");
            }

            /*
            uninstallation process xml START
            */

            // read in xml files
            let str_base_xml = fs::read_to_string(path_sys_base_xml)
                .expect("issue reading system base.xml");
            let str_evdev_xml = fs::read_to_string(path_sys_evdev_xml)
                .expect("issue reading system evdev.xml");
            let str_sla_xml = fs::read_to_string(path_sla_xml)
                .expect("issue reading local sla_layout.xml file");

            // make system xml files parsable
            let ref_str_base_xml = str_base_xml.as_str();
            let ref_str_evdev_xml = str_evdev_xml.as_str();

            // temporary-working-xml-file
            let mut temp_str_base_xml = String::new();

            // check layout footprint
            if (str_base_xml.contains(">sla<")) {
                // if layout exist, skip copying it into the temp_file
                let mut start_str_index = 0;

                // parse through all layouts
                while let Some(layout_start_tag_index) = ref_str_base_xml[start_str_index..].find("<layout>") {
                    // save the layout starting str_index
                    temp_str_base_xml.push_str(&ref_str_base_xml[start_str_index..(start_str_index + layout_start_tag_index)]);
                    // read until the end of layout block
                    if let Some(layout_end_tag_index) = ref_str_base_xml[(start_str_index + layout_start_tag_index)..].find("</layout>") {
                        let layout_content = &ref_str_base_xml[((start_str_index + layout_start_tag_index) + 8)..(start_str_index + layout_start_tag_index + layout_end_tag_index)];
                        if (layout_content.contains(">sla<") == false) {
                            temp_str_base_xml.push_str("<layout>");
                            temp_str_base_xml.push_str(layout_content);
                            temp_str_base_xml.push_str("</layout>");
                        }
                        // else if layout contains target layout, skip copying it into the new file
                        start_str_index = start_str_index + layout_start_tag_index + layout_end_tag_index + 9;
                    } else {
                        // starting layout tag does not have a ending layout tag, file corrupted
                        println!("system base.xml is corrupted, abborting");
                        return;
                    }
                }
                // all layouts parsed, copy everything what is left behind into the temporary-working-xml-file
                temp_str_base_xml.push_str(&ref_str_base_xml[start_str_index..]);
            } else {
                // if file does not contain target layout, just use it as it is
                temp_str_base_xml = str_base_xml;
            }

            /*
            installation process xml START
            */

            // install layout-xml into the temporary-working-xml-file
            // split the temp-xml-file-string right before the first layout
            // and add target xml layout in between the split string
            let (part1_str_base_xml, part2_str_base_xml) = temp_str_base_xml.split_at(temp_str_base_xml.find("<layout>").unwrap());
            let str_new_base_xml = part1_str_base_xml.to_owned() + str_sla_xml.as_str() + part2_str_base_xml;
            // write modified string into the temp-working-xml-file
            fs::write(Path::new(&temp_str_ref_temp_xml_file.clone()), str_new_base_xml.to_owned())
                .expect("issue writing temporary xml file");
            println!("successfully written temporary working xml file");

            // copy and overwrite system xkb and xml files
            let command_copy_xkb = format!("sudo cp -rf {} {}", res_str_ref_xkb_file, sys_str_ref_xkb_file);
            let command_copy_base_xml = format!("sudo cp -rf {} {}", temp_str_ref_temp_xml_file, sys_str_ref_base_xml_file);
            // TODO: may base.xml and evdev.xml are different for some users? -> use second temp_xml
            let command_copy_evdev_xml = format!("sudo cp -rf {} {}", temp_str_ref_temp_xml_file, sys_str_ref_evdev_xml_file);

            // execute installation commands
            let output_cmd_install_sys_xkb = Command::new("sh")
                .arg("-c")
                .arg(command_copy_xkb)
                .output()
                .expect("failed to install xkb file");
            if (output_cmd_install_sys_xkb.status.success() == false) {
                let error_install_sys_xkb = String::from_utf8_lossy(&output_cmd_install_sys_xkb.stderr);
                println!("Error: {}", error_install_sys_xkb);
            } else {
                println!("successfully installed xkb-file")
            }

            let output_cmd_install_sys_base_xml = Command::new("sh")
                .arg("-c")
                .arg(command_copy_base_xml)
                .output()
                .expect("failed to update base.xml file");
            if (output_cmd_install_sys_base_xml.status.success() == false) {
                let error_install_sys_base = String::from_utf8_lossy(&output_cmd_install_sys_base_xml.stderr);
                println!("Error: {}", error_install_sys_base);
            } else {
                println!("successfully updated base.xml")
            }

            let output_cmd_install_sys_evdev_xml = Command::new("sh")
                .arg("-c")
                .arg(command_copy_evdev_xml)
                .output()
                .expect("failed to update evdev.xml file");
            if (output_cmd_install_sys_evdev_xml.status.success() == false) {
                let error_mv_sys_evdev = String::from_utf8_lossy(&output_cmd_install_sys_evdev_xml.stderr);
                println!("Error: {}", error_mv_sys_evdev);
            } else {
                println!("successfully updated evdev.xml")
            }

            // remove temp working file
            fs::remove_file(temp_str_ref_temp_xml_file).expect("issue, while deleting temp.xml file");

            println!("Installation of Modern Slavonic Successfully!");
        }

        "windows" => {
            println!("currently in development, aborting");
            todo!("make use of this program for windows klc files installation");
        }

        _ => {
            println!("unsupported OS, aborting");
            return;
        }
    }
}

use tes3::esp::*;

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let plugin_name;

    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => todo!(),
        2 => plugin_name = &args[1],
        _ => todo!(),
    };

    let mut plugin_to_patch = Plugin::from_path(plugin_name)?;

    for book in plugin_to_patch.objects_of_type_mut::<Book>() {
        if let Some(last_line) = book.text.lines().last() {
            if last_line.to_ascii_lowercase().ends_with("<br>")
                || last_line.to_ascii_lowercase().ends_with("<br>\n")
                || last_line.to_ascii_lowercase().ends_with("<p>")
                || last_line.to_ascii_lowercase().ends_with("<p>\n")
            {
                continue;
            }
            println!(
                "Patching book name: {}, with id: {}",
                book.name,
                book.editor_id()
            );
            book.text += "<BR>\n";
        }
    }

    plugin_to_patch.save_path(format!("patched_{}", plugin_name))?;

    Ok(())
}

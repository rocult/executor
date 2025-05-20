pub fn not_found<S: AsRef<str>>(message: S) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::NotFound, message.as_ref())
}

/// Adds `#include <Windows.h>` to the top of a file if it doesn't already exist.
///
/// Needed for rebasing (e.g. `GetModuleHandle`) to work.
fn add_windows_h_include_top(file: &std::path::Path) -> std::io::Result<()> {
    let file_content = std::fs::read_to_string(file)?;

    // Check if the file already includes <Windows.h>
    if file_content.contains("#include <Windows.h>") {
        return Ok(());
    }

    // Insert the include directive at the top of the file
    let new_content = format!("#include <Windows.h>\n{}", file_content);
    std::fs::write(file, new_content)?;
    Ok(())
}

/// Replaces the body of a function in a file with a new body.
fn replace_func_body(
    file: &std::path::Path,
    func_signature: &str,
    new_func_body: &str,
) -> std::io::Result<()> {
    use std::fs::{read_to_string, write};

    let mut file_content = read_to_string(file)?;

    // Locate the signature and then the opening brace
    let sig_pos = file_content
        .find(func_signature)
        .ok_or_else(|| not_found(format!("Function signature '{func_signature}' not found")))?;
    let body_start = file_content[sig_pos..]
        .find('{')
        .map(|pos| pos + sig_pos)
        .ok_or_else(|| not_found(format!("Opening brace not found for '{func_signature}'")))?;

    // Find closing brace via brace matching
    let mut brace_count = 0;
    let mut body_end = None;
    for (i, ch) in file_content[body_start..].char_indices() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    body_end = Some(body_start + i + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let body_end = body_end.ok_or_else(|| {
        not_found(format!(
            "Could not find matching closing brace for '{func_signature}'",
        ))
    })?;

    // Replace the whole function definition
    let replacement = format!("{func_signature}\n{{\n{new_func_body}\n}}");
    file_content.replace_range(sig_pos..body_end, &replacement);

    write(file, file_content)?;
    Ok(())
}


/// Generates the function body that calls a casted function pointer, based upon the signature and offset.
fn generate_override_body(signature: &str, offset: usize) -> std::io::Result<String> {
    // Find the function name and return type
    let paren_idx = signature
        .find('(')
        .ok_or_else(|| not_found("Invalid signature: missing '('"))?;
    let pre_paren = signature[..paren_idx].trim();
    let mut parts = pre_paren.rsplitn(2, ' ');
    let func_name = parts.next().unwrap();
    let return_type = parts.next().unwrap_or("").trim();

    // Get the parameter list as written in the signature
    let close_paren_idx = signature
        .find(')')
        .ok_or_else(|| not_found("Invalid signature: missing ')'"))?;
    let params_str = signature[paren_idx + 1..close_paren_idx].trim();

    // Extract argument names (assumes parameters are in "type name" format)
    let arg_list = if params_str.is_empty() {
        String::new()
    } else {
        params_str
            .split(',')
            .map(|param| {
                param
                    .trim()
                    .rsplit(|c: char| c.is_whitespace())
                    .next()
                    .unwrap_or("")
                    .trim_matches(|c| c == '*' || c == '&')
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    let typedef_name = format!("{}_off_t", func_name);
    // Build the new function body using hex formatting for the offset
    let output = format!(
        "\ttypedef {} (*{})({});\n\
         \tauto offset = (uintptr_t)GetModuleHandle(nullptr) + {:#x};\n\
         \tauto func = reinterpret_cast<{}>(reinterpret_cast<uintptr_t>(&{}) + offset);\n\
         \treturn func({});",
        return_type, typedef_name, params_str, offset, typedef_name, func_name, arg_list,
    );
    Ok(output)
}

/// Applies function overrides to a list of files.
pub fn do_func_override<'a, P: AsRef<std::path::Path>>(
    overrides: &[(P, &str, usize)],
) -> std::io::Result<()> {
    for (file_path, sig, offset) in overrides {
        add_windows_h_include_top(file_path.as_ref())?;
        let new_body = generate_override_body(sig, *offset)?;
        replace_func_body(file_path.as_ref(), sig, &new_body)?;
    }

    Ok(())
}

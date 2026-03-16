// Rust Tutorial #24: File I/O and Path Handling
// Demonstrates std::fs, Path/PathBuf, BufReader/BufWriter, walking directories, temp files.

use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

// ========================================================
// Path and PathBuf Basics
// ========================================================

fn demonstrate_paths() {
    let path = Path::new("/home/user/documents/report.txt");

    println!("Full path: {}", path.display());
    println!("File name: {:?}", path.file_name());
    println!("Extension: {:?}", path.extension());
    println!("Stem: {:?}", path.file_stem());
    println!("Parent: {:?}", path.parent());
    println!("Is absolute: {}", path.is_absolute());

    // PathBuf is the owned version of Path (like String vs &str)
    let mut buf = PathBuf::from("/home/user");
    buf.push("documents");
    buf.push("report.txt");
    println!("Built path: {}", buf.display());

    // Change extension
    buf.set_extension("pdf");
    println!("Changed extension: {}", buf.display());

    // Join paths
    let base = Path::new("/home/user");
    let full = base.join("documents").join("report.txt");
    println!("Joined: {}", full.display());
}

// ========================================================
// Path Components
// ========================================================

fn path_components(path: &Path) -> Vec<String> {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect()
}

fn path_ancestors(path: &Path) -> Vec<String> {
    path.ancestors()
        .map(|a| a.to_string_lossy().to_string())
        .collect()
}

// ========================================================
// Reading Files
// ========================================================

fn read_entire_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn read_file_bytes(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path)
}

fn read_lines_buffered(path: &Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn read_first_n_lines(path: &Path, n: usize) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<_>, _> = reader.lines().take(n).collect();
    lines
}

fn read_with_capacity(path: &Path) -> io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::with_capacity(8192, file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

// ========================================================
// Writing Files
// ========================================================

fn write_entire_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn write_lines_buffered(path: &Path, lines: &[&str]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    Ok(())
}

fn append_to_file(path: &Path, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

// ========================================================
// File Operations
// ========================================================

fn copy_file(source: &Path, dest: &Path) -> io::Result<u64> {
    fs::copy(source, dest)
}

fn rename_file(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}

fn remove_file_safe(path: &Path) -> io::Result<bool> {
    if path.exists() {
        fs::remove_file(path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn is_file_readonly(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().readonly())
}

// ========================================================
// Directory Operations
// ========================================================

fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

fn list_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        entries.push(entry.path());
    }
    entries.sort();
    Ok(entries)
}

fn list_files_with_extension(dir: &Path, ext: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_ext) = path.extension() {
                if file_ext == ext {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    Ok(files)
}

// ========================================================
// Walking Directories Recursively
// ========================================================

fn walk_directory(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    walk_recursive(dir, &mut result)?;
    result.sort();
    Ok(result)
}

fn walk_recursive(dir: &Path, result: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_recursive(&path, result)?;
            } else {
                result.push(path);
            }
        }
    }
    Ok(())
}

fn find_files_by_name(dir: &Path, name: &str) -> io::Result<Vec<PathBuf>> {
    let all_files = walk_directory(dir)?;
    let matches: Vec<PathBuf> = all_files
        .into_iter()
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().contains(name))
                .unwrap_or(false)
        })
        .collect();
    Ok(matches)
}

fn total_directory_size(dir: &Path) -> io::Result<u64> {
    let files = walk_directory(dir)?;
    let mut total = 0u64;
    for file in &files {
        if let Ok(meta) = fs::metadata(file) {
            total += meta.len();
        }
    }
    Ok(total)
}

// ========================================================
// Temp Files Pattern
// ========================================================

fn create_temp_file(prefix: &str) -> io::Result<PathBuf> {
    let dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("{}_{}.tmp", prefix, timestamp);
    let path = dir.join(name);
    fs::write(&path, "")?;
    Ok(path)
}

// ========================================================
// CSV-like File Processing
// ========================================================

fn read_csv_simple(path: &Path) -> io::Result<Vec<Vec<String>>> {
    let content = fs::read_to_string(path)?;
    let rows: Vec<Vec<String>> = content
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();
    Ok(rows)
}

fn write_csv_simple(path: &Path, rows: &[Vec<String>]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for row in rows {
        writeln!(writer, "{}", row.join(","))?;
    }

    writer.flush()?;
    Ok(())
}

// ========================================================
// Main
// ========================================================

fn main() {
    println!("=== File I/O Demo ===\n");

    // Demo 1: Path operations
    println!("--- Path Operations ---");
    demonstrate_paths();

    println!();

    // Demo 2: Path components
    println!("--- Path Components ---");
    let path = Path::new("/home/user/documents/report.txt");
    let components = path_components(path);
    println!("Components: {:?}", components);

    let ancestors = path_ancestors(path);
    println!("Ancestors: {:?}", ancestors);

    println!();

    // Demo 3: Create temp directory for demos
    println!("--- File Operations ---");
    let temp_dir = std::env::temp_dir().join("rust_file_io_demo");
    let _ = fs::remove_dir_all(&temp_dir);
    create_directory(&temp_dir).expect("Failed to create temp dir");
    println!("Created temp dir: {}", temp_dir.display());

    // Write a file
    let test_file = temp_dir.join("test.txt");
    let content = "Hello, Rust!\nThis is line two.\nAnd line three.\n\nLine five.";
    write_entire_file(&test_file, content).expect("Failed to write");
    println!("Wrote test file");

    // Read it back
    let read_back = read_entire_file(&test_file).expect("Failed to read");
    println!("Read back: {} bytes", read_back.len());
    assert_eq!(read_back, content);

    // Read lines
    let lines = read_lines_buffered(&test_file).expect("Failed to read lines");
    println!("Lines: {}", lines.len());

    // Read first 2 lines
    let first_two = read_first_n_lines(&test_file, 2).expect("Failed to read");
    println!("First 2 lines: {:?}", first_two);

    // File size
    let size = file_size(&test_file).expect("Failed to get size");
    println!("File size: {} bytes", size);

    println!();

    // Demo 4: Buffered writing
    println!("--- Buffered Writing ---");
    let buffered_file = temp_dir.join("buffered.txt");
    write_lines_buffered(&buffered_file, &["alpha", "beta", "gamma"])
        .expect("Failed to write");
    let buffered_content = read_entire_file(&buffered_file).expect("Failed to read");
    println!("Buffered content:\n{}", buffered_content);

    // Demo 5: Append
    println!("--- Append ---");
    append_to_file(&buffered_file, "delta").expect("Failed to append");
    let after_append = read_entire_file(&buffered_file).expect("Failed to read");
    println!("After append:\n{}", after_append);

    println!();

    // Demo 6: Copy and rename
    println!("--- Copy and Rename ---");
    let copy_dest = temp_dir.join("copy.txt");
    let bytes_copied = copy_file(&test_file, &copy_dest).expect("Failed to copy");
    println!("Copied {} bytes", bytes_copied);

    let renamed = temp_dir.join("renamed.txt");
    rename_file(&copy_dest, &renamed).expect("Failed to rename");
    println!("Renamed to: {}", renamed.display());
    assert!(renamed.exists());
    assert!(!copy_dest.exists());

    // Demo 7: Remove
    let removed = remove_file_safe(&renamed).expect("Failed to remove");
    println!("Removed: {}", removed);
    let removed_again = remove_file_safe(&renamed).expect("No error on missing");
    println!("Removed again: {}", removed_again);

    println!();

    // Demo 8: Directory operations
    println!("--- Directory Operations ---");
    let sub_dir = temp_dir.join("subdir");
    create_directory(&sub_dir).expect("Failed to create subdir");
    fs::write(sub_dir.join("a.txt"), "file a").expect("write");
    fs::write(sub_dir.join("b.rs"), "file b").expect("write");
    fs::write(sub_dir.join("c.txt"), "file c").expect("write");

    let entries = list_directory(&temp_dir).expect("Failed to list");
    println!("Directory entries:");
    for e in &entries {
        println!("  {}", e.display());
    }

    let txt_files = list_files_with_extension(&sub_dir, "txt").expect("Failed");
    println!("Text files in subdir: {}", txt_files.len());

    println!();

    // Demo 9: Walk directory
    println!("--- Walk Directory ---");
    let nested = sub_dir.join("nested");
    create_directory(&nested).expect("Failed");
    fs::write(nested.join("deep.txt"), "deep file").expect("write");

    let all_files = walk_directory(&temp_dir).expect("Failed to walk");
    println!("All files:");
    for f in &all_files {
        println!("  {}", f.display());
    }

    let total_size = total_directory_size(&temp_dir).expect("Failed");
    println!("Total size: {} bytes", total_size);

    let found = find_files_by_name(&temp_dir, "deep").expect("Failed");
    println!("Files with 'deep': {:?}", found.iter().map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect::<Vec<_>>());

    println!();

    // Demo 10: CSV
    println!("--- CSV Demo ---");
    let csv_file = temp_dir.join("data.csv");
    let rows = vec![
        vec!["name".to_string(), "age".to_string(), "city".to_string()],
        vec!["Alex".to_string(), "25".to_string(), "Berlin".to_string()],
        vec!["Sam".to_string(), "30".to_string(), "London".to_string()],
    ];
    write_csv_simple(&csv_file, &rows).expect("Failed to write CSV");
    let read_rows = read_csv_simple(&csv_file).expect("Failed to read CSV");
    for row in &read_rows {
        println!("  {:?}", row);
    }

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
    println!("\nCleaned up temp directory");

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "rust_file_io_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("Failed to create test dir");
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_path_components() {
        let path = Path::new("/home/user/file.txt");
        let components = path_components(path);
        assert_eq!(components, vec!["/", "home", "user", "file.txt"]);
    }

    #[test]
    fn test_path_ancestors() {
        let path = Path::new("/home/user/file.txt");
        let ancestors = path_ancestors(path);
        assert_eq!(ancestors.len(), 4);
        assert_eq!(ancestors[0], "/home/user/file.txt");
        assert_eq!(ancestors[1], "/home/user");
        assert_eq!(ancestors[2], "/home");
        assert_eq!(ancestors[3], "/");
    }

    #[test]
    fn test_write_and_read_file() {
        let dir = setup_temp_dir();
        let file = dir.join("test.txt");

        write_entire_file(&file, "hello world").unwrap();
        let content = read_entire_file(&file).unwrap();
        assert_eq!(content, "hello world");

        cleanup(&dir);
    }

    #[test]
    fn test_read_file_bytes() {
        let dir = setup_temp_dir();
        let file = dir.join("bytes.txt");

        fs::write(&file, b"hello").unwrap();
        let bytes = read_file_bytes(&file).unwrap();
        assert_eq!(bytes, b"hello");

        cleanup(&dir);
    }

    #[test]
    fn test_read_lines_buffered() {
        let dir = setup_temp_dir();
        let file = dir.join("lines.txt");

        fs::write(&file, "line1\nline2\nline3").unwrap();
        let lines = read_lines_buffered(&file).unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);

        cleanup(&dir);
    }

    #[test]
    fn test_read_first_n_lines() {
        let dir = setup_temp_dir();
        let file = dir.join("lines.txt");

        fs::write(&file, "a\nb\nc\nd\ne").unwrap();
        let first_two = read_first_n_lines(&file, 2).unwrap();
        assert_eq!(first_two, vec!["a", "b"]);

        cleanup(&dir);
    }

    #[test]
    fn test_read_with_capacity() {
        let dir = setup_temp_dir();
        let file = dir.join("cap.txt");

        fs::write(&file, "buffered content").unwrap();
        let content = read_with_capacity(&file).unwrap();
        assert_eq!(content, "buffered content");

        cleanup(&dir);
    }

    #[test]
    fn test_write_lines_buffered() {
        let dir = setup_temp_dir();
        let file = dir.join("buffered.txt");

        write_lines_buffered(&file, &["alpha", "beta"]).unwrap();
        let content = read_entire_file(&file).unwrap();
        assert_eq!(content, "alpha\nbeta\n");

        cleanup(&dir);
    }

    #[test]
    fn test_append_to_file() {
        let dir = setup_temp_dir();
        let file = dir.join("append.txt");

        fs::write(&file, "first\n").unwrap();
        append_to_file(&file, "second").unwrap();
        let content = read_entire_file(&file).unwrap();
        assert!(content.contains("first"));
        assert!(content.contains("second"));

        cleanup(&dir);
    }

    #[test]
    fn test_append_creates_file() {
        let dir = setup_temp_dir();
        let file = dir.join("new_append.txt");

        append_to_file(&file, "hello").unwrap();
        assert!(file.exists());
        let content = read_entire_file(&file).unwrap();
        assert!(content.contains("hello"));

        cleanup(&dir);
    }

    #[test]
    fn test_copy_file() {
        let dir = setup_temp_dir();
        let src = dir.join("src.txt");
        let dest = dir.join("dest.txt");

        fs::write(&src, "copy me").unwrap();
        let bytes = copy_file(&src, &dest).unwrap();
        assert_eq!(bytes, 7);
        assert_eq!(read_entire_file(&dest).unwrap(), "copy me");

        cleanup(&dir);
    }

    #[test]
    fn test_rename_file() {
        let dir = setup_temp_dir();
        let from = dir.join("old.txt");
        let to = dir.join("new.txt");

        fs::write(&from, "rename me").unwrap();
        rename_file(&from, &to).unwrap();
        assert!(!from.exists());
        assert!(to.exists());
        assert_eq!(read_entire_file(&to).unwrap(), "rename me");

        cleanup(&dir);
    }

    #[test]
    fn test_remove_file_safe_exists() {
        let dir = setup_temp_dir();
        let file = dir.join("remove.txt");

        fs::write(&file, "delete me").unwrap();
        let removed = remove_file_safe(&file).unwrap();
        assert!(removed);
        assert!(!file.exists());

        cleanup(&dir);
    }

    #[test]
    fn test_remove_file_safe_not_exists() {
        let path = Path::new("/tmp/nonexistent_file_xyz_123.txt");
        let removed = remove_file_safe(path).unwrap();
        assert!(!removed);
    }

    #[test]
    fn test_file_size() {
        let dir = setup_temp_dir();
        let file = dir.join("sized.txt");

        fs::write(&file, "12345").unwrap();
        let size = file_size(&file).unwrap();
        assert_eq!(size, 5);

        cleanup(&dir);
    }

    #[test]
    fn test_create_directory() {
        let dir = setup_temp_dir();
        let nested = dir.join("a").join("b").join("c");

        create_directory(&nested).unwrap();
        assert!(nested.exists());
        assert!(nested.is_dir());

        cleanup(&dir);
    }

    #[test]
    fn test_list_directory() {
        let dir = setup_temp_dir();

        fs::write(dir.join("a.txt"), "a").unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        fs::create_dir(dir.join("subdir")).unwrap();

        let entries = list_directory(&dir).unwrap();
        assert_eq!(entries.len(), 3);

        cleanup(&dir);
    }

    #[test]
    fn test_list_files_with_extension() {
        let dir = setup_temp_dir();

        fs::write(dir.join("a.txt"), "a").unwrap();
        fs::write(dir.join("b.rs"), "b").unwrap();
        fs::write(dir.join("c.txt"), "c").unwrap();

        let txt_files = list_files_with_extension(&dir, "txt").unwrap();
        assert_eq!(txt_files.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn test_walk_directory() {
        let dir = setup_temp_dir();
        let sub = dir.join("sub");
        fs::create_dir(&sub).unwrap();

        fs::write(dir.join("root.txt"), "root").unwrap();
        fs::write(sub.join("child.txt"), "child").unwrap();

        let files = walk_directory(&dir).unwrap();
        assert_eq!(files.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn test_find_files_by_name() {
        let dir = setup_temp_dir();
        let sub = dir.join("sub");
        fs::create_dir(&sub).unwrap();

        fs::write(dir.join("config.txt"), "").unwrap();
        fs::write(sub.join("config.json"), "").unwrap();
        fs::write(sub.join("data.txt"), "").unwrap();

        let found = find_files_by_name(&dir, "config").unwrap();
        assert_eq!(found.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn test_total_directory_size() {
        let dir = setup_temp_dir();

        fs::write(dir.join("a.txt"), "12345").unwrap(); // 5 bytes
        fs::write(dir.join("b.txt"), "123").unwrap(); // 3 bytes

        let total = total_directory_size(&dir).unwrap();
        assert_eq!(total, 8);

        cleanup(&dir);
    }

    #[test]
    fn test_create_temp_file() {
        let path = create_temp_file("test_prefix").unwrap();
        assert!(path.exists());
        assert!(path.file_name().unwrap().to_string_lossy().starts_with("test_prefix_"));
        assert!(path.file_name().unwrap().to_string_lossy().ends_with(".tmp"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_csv_roundtrip() {
        let dir = setup_temp_dir();
        let file = dir.join("test.csv");

        let rows = vec![
            vec!["name".to_string(), "age".to_string()],
            vec!["Alex".to_string(), "25".to_string()],
        ];

        write_csv_simple(&file, &rows).unwrap();
        let read_back = read_csv_simple(&file).unwrap();
        assert_eq!(read_back.len(), 2);
        assert_eq!(read_back[0], vec!["name", "age"]);
        assert_eq!(read_back[1], vec!["Alex", "25"]);

        cleanup(&dir);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_entire_file(Path::new("/tmp/nonexistent_xyz_123.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_is_file_readonly() {
        let dir = setup_temp_dir();
        let file = dir.join("readonly_test.txt");
        fs::write(&file, "test").unwrap();

        let readonly = is_file_readonly(&file).unwrap();
        assert!(!readonly);

        cleanup(&dir);
    }
}

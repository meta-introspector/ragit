# SOP: Grep and Sed for Code Refactoring

## 1. Purpose
This Standard Operating Procedure (SOP) outlines a systematic approach to refactoring code using `grep` for identification and `sed` for automated text manipulation. This method is particularly useful for applying consistent changes across multiple files or for extracting code blocks based on specific patterns, adhering to the "one declaration per file" principle.

## 2. Scope
This SOP applies to all text-based code files within the `ragit` project where repetitive or pattern-based refactoring is required. It is especially relevant for tasks like:
*   Extracting functions, structs, or `impl` blocks into separate files.
*   Modifying import paths or declarations.
*   Adding or removing boilerplate code.

## 3. Procedure

### 3.1. Understand the Change
Clearly define the refactoring task. Identify the exact code pattern to be moved, modified, or deleted, and the desired outcome.

### 3.2. Identify Target Files and Code Blocks using `grep`

#### 3.2.1. Initial Identification
Use `grep` to locate all occurrences of the target code pattern.
```bash
grep -nE 
'<pattern>'
 <file_path_or_glob_pattern>
```
*   `-n`: Displays line numbers, crucial for `sed` operations.
*   `-E`: Enables extended regular expressions.

#### 3.2.2. Refine Search (if necessary)
If the initial `grep` returns too many irrelevant results, refine the pattern or use additional `grep` options (e.g., `-w` for whole word, `-A <num>` for after context, `-B <num>` for before context).

#### 3.2.3. Determine Line Ranges
For code extraction, carefully identify the start and end line numbers of the code block to be moved. This often requires manual inspection of the `grep` output and the surrounding code.

### 3.3. Extract Code Blocks using `sed` (for "One Declaration Per File")

#### 3.3.1. Extract to New File
Use `sed` to extract the identified lines into a new file.
```bash
sed -n 
'<start_line>,<end_line>p'
 <source_file> > <destination_file>
```
*   `-n`: Suppresses automatic printing of lines.
*   `p`: Prints the matched lines.

#### 3.3.2. Add Metadata Header (if applicable)
If the extracted file requires a metadata header (as per `documenting_cpu_backend_operations_refactoring.md`), use `sed` to prepend it. Remember to escape special characters and newlines.
```bash
sed -i 
'1i\\\n<line1>\\\n<line2>\\\n...
'
 <destination_file>
```
*   `-i`: Edits the file in place.
*   `1i`: Inserts text before the first line.
*   `\\\n`: Inserts a newline.
*   `\\`: Escapes special characters (e.g., `/` in paths or emojis).

#### 3.3.3. Delete from Source File
After successful extraction, delete the original lines from the source file.
```bash
sed -i 
'<start_line>,<end_line>d'
 <source_file>
```
*   `d`: Deletes the matched lines.

### 3.4. Modify Code using `sed` (for in-place changes)

#### 3.4.1. Simple String Replacement
For direct string replacements (e.g., updating import paths).
```bash
sed -i 
's/<old_string>/<new_string>/g'
 <file_path>
```
*   `s`: Substitute command.
*   `g`: Global replacement (replaces all occurrences on a line).

#### 3.4.2. Line Insertion/Deletion
For inserting new lines or deleting specific lines based on content.
```bash
# Insert after a pattern
sed -i 
'/^<pattern>$/a <new_line>'
 <file_path>
# Delete lines matching a pattern
sed -i 
'/^<pattern>$/d'
 <file_path>
```

### 3.5. Update `mod.rs` (if applicable)
If code was extracted, update the `mod.rs` file in the parent directory to include `pub mod <new_module_name>;` or `pub use <new_module_name>::*;
` statements.

### 3.6. Iterative Process and Verification
*   Perform changes incrementally. After each major `sed` operation, verify the changes.
*   Run `cargo build` frequently to catch compilation errors early.
*   Use `git diff` to review changes before committing.

## 4. Tools
*   `grep`
*   `sed`
*   `git` (for version control and diffing)
*   `cargo` (for building and checking)

## 5. Best Practices
*   **Backup**: Always commit your work or create a backup before performing extensive `sed` operations.
*   **Test**: Thoroughly test the refactored code after changes.
*   **Specificity**: Be as specific as possible with `grep` patterns and `sed` commands to avoid unintended modifications.
*   **Escaping**: Pay close attention to escaping special characters (e.g., `/`, `[`, `]`, `*`, `.` in regex, and `\` for newlines in `sed` insertion).
*   **Line Numbers**: Be aware that line numbers shift after deletions or insertions. Re-`grep` for line numbers if performing multiple sequential operations on the same file.

## 6. Expected Outcome
*   Efficient and accurate code refactoring.
*   Consistent application of code style and structure.
*   Reduced manual effort for repetitive tasks.
*   Improved code quality and maintainability.

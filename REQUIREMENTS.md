rshell TODO Checklist

1. Project Setup
- [ ] Create Rust project with cargo new rshell
- [ ] Configure rustfmt
- [ ] Configure clippy
- [ ] Create module structure
- [ ] Create initial README

2. REPL Loop
- [ ] Display shell prompt
- [ ] Read user input from stdin
- [ ] Handle empty input
- [ ] Implement infinite REPL loop
- [ ] Gracefully terminate shell

3. Basic Command Execution
- [ ] Parse command and arguments
- [ ] Spawn child processes
- [ ] Execute external binaries
- [ ] Wait for child process completion
- [ ] Display execution errors
- [ ] Handle invalid commands

4. Built-in Commands
- [ ] cd
    - Implement cd
    - Handle invalid directories
    - Support cd ..
    - Support cd ~
- [ ] pwd
- [ ] echo
- [ ] clear
- [ ] exit

5. Lexer
- [ ] Token Infrastructure
    - Create token enum
    - Implement lexer state machine
    - Tokenize plain words
- [ ] Quotes
    - Support double quotes
    - Support single quotes
    - Handle unterminated quotes
- [ ] Escaping
    - Support escaped characters
    - Support escaped spaces
- [ ] Operators
    - Tokenize |
    - Tokenize >
    - Tokenize >>
    - Tokenize <
    - Tokenize &&
    - Tokenize ||
    - Tokenize &
- [ ] Testing

6. Parser & AST
- [ ] AST
    - Design AST node types
    - Implement command AST nodes
    - Implement pipe AST nodes
    - Implement redirect AST nodes
    - Implement logical operator AST nodes
- [ ] Parser
    - Parse simple commands
    - Parse command arguments
    - Parse pipes
    - Parse redirects
    - Parse chained commands
    - Parse background jobs
    - Handle parser errors
    - Detect invalid syntax
- [ ] Testing

7. Pipes
- [ ] Create Unix pipes
- [ ] Redirect stdout to pipe
- [ ] Redirect stdin from pipe
- [ ] Execute piped commands
- [ ] Support multi-stage pipes
- [ ] Close unused file descriptors
- [ ] Handle pipe failures
- [ ] Test pipe behavior

8. Redirection
- [ ] Output Redirection
    - Implement >
    - Create output files
    - Truncate existing files
- [ ] Append Redirection
    - Implement >>
    - Append to files
- [ ] Input Redirection
    - Implement <
    - Read input from files
- [ ] File Descriptor Management
    - Duplicate file descriptors
    - Restore original descriptors
    - Handle redirection errors
- [ ] Testing
    - Test stdout redirection
    - Test stdin redirection
    - Test append behavior

9. Environment Variables
- [ ] Variables
    - Read environment variables
    - Expand $VAR, $HOME, $PATH, etc.
- [ ] Export
    - Implement export
    - Store shell variables
    - Inherit env vars in child processes
- [ ] Edge Cases
    - Handle undefined variables
    - Handle variables in quotes
- [ ] Testing

10. Command Chaining
- [ ] Logical Operators
    - Implement &&
    - Implement ||
- [ ] Exit Status Logic
    - Track process exit codes
- [ ] Testing

11. Background Jobs
- [ ] Background Execution
    - Detect &
    - Run jobs asynchronously
    - Avoid blocking shell
- [ ] Job Tracking
    - Create job table
    - Store job IDs
    - Track process states
- [ ] Built-ins
    - Implement jobs
    - Implement fg
    - Implement bg
- [ ] Process Management
    - Reap background processes
    - Prevent zombie processes
- [ ] Testing

12. Signal Handling
- [ ] SIGINT
    - Handle Ctrl+C
    - Prevent shell termination on SIGINT
    - Forward SIGINT to foreground process
- [ ] SIGTSTP
    - Handle Ctrl+Z
    - Suspend foreground jobs
- [ ] Process Groups
    - Create process groups
    - Transfer terminal control
    - Restore terminal ownership
- [ ] Handle Ctrl+D (EOF)
- [ ] Testing

13. Command History
- [ ] History Storage
    - Store executed commands
    - Persist history to disk
    - Load history on startup
- [ ] Navigation
    - Support up-arrow navigation
    - Support down-arrow navigation
- [ ] Built-ins
    - Implement history
- [ ] Testing

14. Tab Completion
- [ ] File Completion
    - Complete filenames
    - Complete directories
- [ ] Command Completion
    - Complete executable names
    - Complete built-in commands
- [ ] UX Improvements
    - Display multiple suggestions
    - Handle partial matches
- [ ] Testing

15. Configuration System
- [ ] Config Loading
    - Load ~/.rshellrc
    - Parse config commands
    - Execute startup commands
- [ ] Aliases
    - Implement aliases
    - Expand aliases during parsing
- [ ] Environment
    - Load exported variables from config
- [ ] Testing

16. Stretch Goals
- [ ] Shell Features
    - Subshells
    - Command substitution
    - Shell scripting support
    - Functions
    - Conditional statements
    - Loops
- [ ] UX Improvements
    - Syntax highlighting
    - Autosuggestions
    - Colored prompt
    - Git branch prompt

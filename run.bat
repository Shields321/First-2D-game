@echo off
cargo run --bin game

set /p userInput=Enter your choice (clean/exit) clean will make starting longer but will take up less storage: 
if /i "%userInput%"=="clean" (    
    cargo clean    
) else if /i "%userInput%"=="exit" (    
    goto :EOF
) else (
    echo Invalid choice. Please enter 'start' or 'exit'.
)
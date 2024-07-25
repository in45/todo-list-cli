# Todo List CLI Application

## Description

This is a simple command-line interface (CLI) application for managing a todo list. Built in Rust, it allows users to add tasks, mark tasks as completed, list all tasks, and remove tasks. It utilizes JSON serialization to persist tasks between runs.

## Features

- Add a new task
- Mark a task as completed
- List all tasks
- Remove a specific task

## Dependencies

- `serde` for serialization and deserialization
- `serde_json` for handling JSON data
- `clap` for command-line argument parsing

## Build The Project
```bash
cargo build --release
```
## Usage
Run the application using the following commands:

- Add a Task:
```bash
./target/release/todo-cli add "Your task description"
```
- Mark Task as Completed:
```bash
./target/release/todo-cli complete <task_index>
```
- Remove a Task:
```bash
./target/release/todo-cli remove <task_index>
```
- List All Tasks:
```bash
./target/release/todo-cli list
```

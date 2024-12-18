use dialoguer::{console::Term, theme::ColorfulTheme, Select, Input};
use std::process::Command;

fn main() {
    let options = vec![
        "ステージング (git add)", 
        "コミット (git commit)", 
        "プッシュ (git push)", 
        "ステータス確認 (git status)", 
        "ブランチ作成 (git branch)"
    ];

    let term = Term::stderr();
    println!("Git Wizardへようこそ！\n");

    loop {
        // メニューの選択
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("実行したいGitコマンドを選んでください (:q で終了)")
            .items(&options)
            .default(0)
            .interact_on_opt(&term)
            .unwrap();

        match selection {
            Some(index) => match index {
                0 => {
                    // git add
                    let files = user_input_with_exit(&term, "ステージングしたいファイル/ディレクトリを指定してください (例: .)");
                    execute_command("git", &["add", &files]);
                }
                1 => {
                    // git commit
                    let message = user_input_with_exit(&term, "コミットメッセージを入力してください");
                    execute_command("git", &["commit", "-m", &message]);
                }
                2 => {
                    // git push
                    execute_command("git", &["push"]);
                }
                3 => {
                    // git status
                    execute_command("git", &["status"]);
                }
                4 => {
                    // git branch
                    let branch_name = user_input_with_exit(&term, "新しいブランチ名を入力してください");
                    execute_command("git", &["branch", &branch_name]);
                }
                _ => {
                    println!("不明な選択肢です。");
                }
            },
            None => {
                // 選択をキャンセル（Ctrl+CやESCが押された場合）
                println!("終了します。");
                break;
            }
        }
    }
}

/// ユーザー入力を受け取るヘルパー関数 (:qで終了対応)
fn user_input_with_exit(term: &Term, prompt: &str) -> String {
    loop {
        let input: String = Input::new()
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input.trim() == ":q" {
            println!("終了します。");
            term.clear_last_lines(1).unwrap();
            std::process::exit(0);
        }

        if !input.trim().is_empty() {
            return input;
        }
    }
}

/// コマンドを実行するヘルパー関数
fn execute_command(command: &str, args: &[&str]) {
    println!("\n実行中: {} {}\n", command, args.join(" "));
    let output = Command::new(command)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("コマンドの実行に失敗しました: {}", e);
        }
    }
}


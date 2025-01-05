pub fn get_linux_excludes() -> Vec<String> {
    let username = env::var("USER").unwrap_or_else(|_| whoami::username());
    vec![
        "./**/*.sock".to_string(),
        "./.gnupg/S.*".to_string(),
        format!("./{}/Sources/github.com/NixOS/nixpkgs", username),
        // Add Neovim config and data
        format!("./{}/local/share/nvim", username),
    ]
}

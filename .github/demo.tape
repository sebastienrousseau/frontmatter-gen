# SPDX-License-Identifier: Apache-2.0 OR MIT
# VHS tape for generating .github/demo.gif
# Run with: vhs .github/demo.tape (or: make demo)

Output .github/demo.gif

Require fmg

Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 640
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms
Set Framerate 30

Hide
Type "export PS1='$ '"
Enter
Type 'DEMO_DIR=$(mktemp -d /tmp/fmg-demo.XXXX)'
Enter
Type 'trap "rm -rf \"$DEMO_DIR\"" EXIT'
Enter
Type 'cat > "$DEMO_DIR/post.md" << "EOF"'
Enter
Type '---'
Enter
Type 'title: "Deterministic Front Matter in Rust"'
Enter
Type 'author: "Sebastien Rousseau"'
Enter
Type 'date: "2026-04-21"'
Enter
Type 'tags: ["rust", "yaml", "toml", "json"]'
Enter
Type 'draft: false'
Enter
Type '---'
Enter
Type '# Hello, World!'
Enter
Type 'EOF'
Enter
Type 'cd "$DEMO_DIR" && clear'
Enter
Sleep 300ms
Show

Type "# 1. Inspect the Markdown document with YAML front matter"
Enter
Sleep 300ms
Type "cat post.md"
Enter
Sleep 1.8s

Type "# 2. Validate required front matter fields with fmg"
Enter
Sleep 300ms
Type "fmg validate post.md --required title,author,date"
Enter
Sleep 1.5s

Type "# 3. Extract and convert front matter to JSON and TOML"
Enter
Sleep 300ms
Type "fmg extract post.md --format json --output metadata.json && cat metadata.json && echo"
Enter
Sleep 2s

Type "fmg extract post.md --format toml --output metadata.toml && cat metadata.toml"
Enter
Sleep 3s

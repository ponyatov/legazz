# line comment

# dirs

mkdir .vscode bin doc lib inc src tmp

# files

touch Makefile apt.txt .doxygen .clang-format apt.txt

# giti

echo '*~'           >     .gitignore
echo '*.swp'       >>     .gitignore
echo '*.log'       >>     .gitignore
echo ''            >>     .gitignore
echo '/target/'    >>     .gitignore
echo '/Cargo.lock' >>     .gitignore

echo  '*'           > bin/.gitignore
echo  '*.pdf'       > doc/.gitignore
touch                 lib/.gitignore
touch                 inc/.gitignore
touch                 src/.gitignore
echo  '*'           > tmp/.gitignore

# VSCode

touch .vscode/extensions.json
touch .vscode/tasks.json
touch .vscode / settings.json

# VSCode plugin

touch                  .vscode/package.json
touch                  .vscode/extension.js
touch                  .vscode/lega.tmLanguage.json
touch                  .vscode/lega.configuration.json
touch                  .vscode/snippets.json
ln -fs ../doc/logo.png .vscode/logo.png

# VM

touch src/main.rs
touch src/lexer.rs
touch src/parser.rs

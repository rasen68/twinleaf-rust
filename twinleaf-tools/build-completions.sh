#!/usr/bin/env bash
# This should be run after any changes to completions or CLI

echo "Initial build..."
cargo build
TIO=../target/debug/tio
COMP=completion-scripts/tio_completions

# generate static scripts
echo "Generating static scripts"
"$TIO" completions --static bash > "$COMP"_static.bash || exit 1
"$TIO" completions --static zsh > "$COMP"_static.zsh || exit 1
echo "Re-build..."
cargo build # build again to embed static scripts

# generate dynamic scripts
echo "Generating dynamic scripts"
"$TIO" completions bash > "$COMP"_dynamic.bash || exit 1
"$TIO" completions zsh > "$COMP"_dynamic.zsh || exit 1

echo "Testing generated scripts"
bash -n "$COMP"_dynamic.bash || exit 1
zsh -n "$COMP"_dynamic.zsh || exit 1

if [[ "$1" == "--commit" ]]; then
	echo "Creating git commit"
	git add "$COMP"*
	git commit -m "Chore: Regenerate completion scripts"
fi

#!/bin/sh

set -e

if ! command -v unzip >/dev/null; then
	echo "Error: unzip is required to install dt" 1>&2
	exit 1
fi

if [ "$OS" = "Windows_NT" ]; then
	target="windows-2022"
else
	case $(uname -sm) in
	"Darwin x86_64") target="macos-11" ;;
	"Darwin arm64") target="macos-11" ;;
	*) target="ubuntu-22.04" ;;
	esac
fi

dt_uri=""

if [ $# -eq 0 ]; then
	dt_uri="https://github.com/dtemplate/dt/releases/latest/download/dt-${target}.zip"
else
	dt_uri="https://github.com/dtemplate/dt/releases/download/${1}/dt-${target}.zip"
fi


dt_install="${DT_INSTALL:-$HOME/.dt}"
bin_dir="$dt_install/bin"
exe="$bin_dir/dt"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.zip" "$dt_uri"
unzip -d "$bin_dir" -o "$exe.zip"
chmod +x "$exe"
rm "$exe.zip"

echo "dt was installed successfully to $exe"
if command -v dt >/dev/null; then
	echo "Run 'dt --help' to get started"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bashrc" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export DT_INSTALL=\"$dt_install\""
	echo "  export PATH=\"\$DT_INSTALL/bin:\$PATH\""
	echo "Run '$exe --help' to get started"
fi

rustup override set stable-x86_64-pc-windows-msvc
cargo build --release
copy target\release\c_tokenizers.dll .\gotokenizer\c_tokenizers.dll
pause
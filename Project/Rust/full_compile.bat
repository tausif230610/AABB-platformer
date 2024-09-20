cls
echo Full Compilation is About to Begain K
pause
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=optimize_for_size -Z build-std-features=panic_immediate_abort  --target x86_64-pc-windows-msvc
cls
echo Debug x86_64 msvc aka windows done
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=optimize_for_size -Z build-std-features=panic_immediate_abort  --target x86_64-pc-windows-msvc --release
cls
echo release x86_64 msvc aka windows done

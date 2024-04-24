# Ton client for flutter

## Build Android:
- install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- open ../ton_client/Cargo.toml #86, open openssl dependence.
- execute:
- `./buildAndroid`

## Build-iOS:
- open ../ton_client/Cargo.toml #86, close openssl dependence.
- execute:
- `cargo build --target aarch64-apple-ios --release`
- `cargo build --target x86_64-apple-ios --release`

### Generate Header file for iOS
- install `cargo install cbindgen`: https://docs.rs/crate/cbindgen/latest
- generate header file
- `./generate-header`
- append to tonclient.h
```c
#if defined(__cplusplus)
#include <cstdint>
#include <cstddef>
#include <cstdbool>
#include <cstring>
#include <stdlib.h>
#include <new>
#include <type_traits>
#else
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <string.h>
#endif
```

## copy content to xc framework

- copy header
`cp tonclient.h TonClient.xcframework/ios-arm64/TonClient.framework/Headers/TonClient.h`
`cp tonclient.h TonClient.xcframework/ios-arm64_x86_64-simulator/TonClient.framework/Headers/TonClient.h`

- copy .a
`cp ../target/aarch64-apple-ios/release/libton_client_dart.a TonClient.xcframework/ios-arm64/TonClient.framework/TonClient`
`cp ../target/x86_64-apple-ios/release/libton_client_dart.a TonClient.xcframework/ios-arm64_x86_64-simulator/TonClient.framework/TonClient`


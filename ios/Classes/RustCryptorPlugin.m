#import "RustCryptorPlugin.h"
#if __has_include(<rust_cryptor/rust_cryptor-Swift.h>)
#import <rust_cryptor/rust_cryptor-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "rust_cryptor-Swift.h"
#endif

@implementation RustCryptorPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  ((void*)dummy_method_to_enforce_bundling);
  [SwiftRustCryptorPlugin registerWithRegistrar:registrar];
}
@end

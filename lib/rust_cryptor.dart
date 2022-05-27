import 'dart:ffi';
import 'dart:io';

import 'package:rust_cryptor/bridge_generated.dart';

import 'rust_cryptor_platform_interface.dart';

class RustCryptor {
  final RustImpl api;

  factory RustCryptor() {
    const base = 'rust';
    final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
    late final dylib = Platform.isIOS
        ? DynamicLibrary.process()
        : Platform.isMacOS
            ? DynamicLibrary.executable()
            : DynamicLibrary.open(path);
    final RustImpl api = RustImpl(dylib);
    return RustCryptor._(api);
  }

  const RustCryptor._(this.api);

  Future<String?> getPlatformVersion() {
    return RustCryptorPlatform.instance.getPlatformVersion();
  }

  Future<String?> greet() {
    return api.greet();
  }
}

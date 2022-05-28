import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:rust_cryptor/bridge_generated.dart';

import 'rust_cryptor_platform_interface.dart';

class RustLib {
  final RustImpl api;

  factory RustLib() {
    const base = 'rust';
    final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
    late final dylib = Platform.isIOS
        ? DynamicLibrary.process()
        : Platform.isMacOS
            ? DynamicLibrary.executable()
            : DynamicLibrary.open(path);
    final RustImpl api = RustImpl(dylib);
    return RustLib._(api);
  }

  const RustLib._(this.api);

  Future<String?> getPlatformVersion() {
    return RustCryptorPlatform.instance.getPlatformVersion();
  }

  Future<String?> greet() {
    return api.greet();
  }
}

class RustCryptor {
  static const _max = 1 << 30;
  final RustLib _lib;
  final CryptorHandle _handle;

  static Future<RustCryptor> create(String key, int ivLength) async {
    final lib = RustLib();
    final handle = await lib.api.cryptorNew(
      subPoolId: Random().nextInt(_max),
      key: Uint8List.fromList(utf8.encode(key)),
      ivLength: ivLength,
    );
    return RustCryptor._(lib, handle);
  }

  const RustCryptor._(this._lib, this._handle);

  Future<Uint8List> encrypt(String data) {
    return _lib.api.cryptorEncrypt(cryptor: _handle, text: data);
  }

  Future<String> decrypt(Uint8List data) {
    return _lib.api.cryptorDecrypt(cryptor: _handle, data: data);
  }

  Future<String?> greet() {
    return _lib.greet();
  }

  Future<void> dispose() {
    return _lib.api.cryptorRemove(cryptor: _handle);
  }
}

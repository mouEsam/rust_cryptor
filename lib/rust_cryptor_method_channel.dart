import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'rust_cryptor_platform_interface.dart';

/// An implementation of [RustCryptorPlatform] that uses method channels.
class MethodChannelRustCryptor extends RustCryptorPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('rust_cryptor');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}

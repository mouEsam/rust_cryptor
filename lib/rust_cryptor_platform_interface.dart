import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'rust_cryptor_method_channel.dart';

abstract class RustCryptorPlatform extends PlatformInterface {
  /// Constructs a RustCryptorPlatform.
  RustCryptorPlatform() : super(token: _token);

  static final Object _token = Object();

  static RustCryptorPlatform _instance = MethodChannelRustCryptor();

  /// The default instance of [RustCryptorPlatform] to use.
  ///
  /// Defaults to [MethodChannelRustCryptor].
  static RustCryptorPlatform get instance => _instance;
  
  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [RustCryptorPlatform] when
  /// they register themselves.
  static set instance(RustCryptorPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}


import 'rust_cryptor_platform_interface.dart';

class RustCryptor {
  Future<String?> getPlatformVersion() {
    return RustCryptorPlatform.instance.getPlatformVersion();
  }
}

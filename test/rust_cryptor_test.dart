import 'package:flutter_test/flutter_test.dart';
import 'package:rust_cryptor/rust_cryptor.dart';
import 'package:rust_cryptor/rust_cryptor_platform_interface.dart';
import 'package:rust_cryptor/rust_cryptor_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockRustCryptorPlatform 
    with MockPlatformInterfaceMixin
    implements RustCryptorPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final RustCryptorPlatform initialPlatform = RustCryptorPlatform.instance;

  test('$MethodChannelRustCryptor is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelRustCryptor>());
  });

  test('getPlatformVersion', () async {
    RustCryptor rustCryptorPlugin = RustCryptor();
    MockRustCryptorPlatform fakePlatform = MockRustCryptorPlatform();
    RustCryptorPlatform.instance = fakePlatform;
  
    expect(await rustCryptorPlugin.getPlatformVersion(), '42');
  });
}

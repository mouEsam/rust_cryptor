import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'dart:async';

import 'package:flutter/services.dart';
import 'package:rust_cryptor/rust_cryptor.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String _platformVersion = 'Unknown';
  String _result = 'Unknown';
  RustCryptor? _rustCryptorPlugin;
  final TextEditingController controller = TextEditingController();

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  @override
  void dispose() {
    _rustCryptorPlugin?.dispose();
    controller.dispose();
    super.dispose();
  }

  Future<RustCryptor> ensureInit() async {
    _rustCryptorPlugin ??= await RustCryptor.create('WmZq4t7w!z%C*F-J', 16);
    return _rustCryptorPlugin!;
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initPlatformState() async {
    String platformVersion;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      final cryptor = await ensureInit();
      platformVersion = await cryptor.greet() ?? 'Unknown platform version';
    } on PlatformException {
      platformVersion = 'Failed to get platform version.';
    }

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;

    setState(() {
      _platformVersion = platformVersion;
    });
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> encrypt() async {
    final text = controller.text;
    Uint8List result;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      final cryptor = await ensureInit();
      result = await cryptor.encrypt(text);
    } on PlatformException {
      result = Uint8List.fromList('Failed to get platform version.'.codeUnits);
    }

    print("GOt gere");

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;

    setState(() {
      _result = String.fromCharCodes(result);
    });
  }

  @override
  Widget build(BuildContext context) {
    const spacer = SizedBox(
      height: 16,
    );
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'Message: $_platformVersion\n',
                textAlign: TextAlign.center,
              ),
              spacer,
              Text(
                'Result: $_result\n',
                textAlign: TextAlign.center,
              ),
              spacer,
              TextField(
                controller: controller,
              ),
              spacer,
              ElevatedButton(
                onPressed: encrypt,
                child: const Text('Encrypt'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

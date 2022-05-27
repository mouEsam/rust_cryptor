#include "include/rust_cryptor/rust_cryptor_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "rust_cryptor_plugin.h"

void RustCryptorPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  rust_cryptor::RustCryptorPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}

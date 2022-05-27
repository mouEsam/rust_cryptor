message("-- Linking Rust")
set(CRATE_NAME "rust")
set(CRATE_NAME ${CRATE_NAME} PARENT_SCOPE)
add_library(${CRATE_NAME} SHARED IMPORTED GLOBAL)
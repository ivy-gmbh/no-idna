# no-idna

This crate allows you to patch the idna crate in rust-url in order to reduce the binary size.
Doing so means that parsing URLs containing unicode characters will fail.

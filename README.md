# Vaxuum

Vaxuum is a utility to translate files from VMS Variable length records to a typical \*nix-style file. In VMS parlance,
this translates variable records into undefined records, where every file is just a stream of bytes from beginning
to the end.

# Usage

```bash
cargo run -- <file> [file] ...
```

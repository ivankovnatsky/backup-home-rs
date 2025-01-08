# backup-home

## Development

### Prerequisites

- [direnv](https://direnv.net/)
- [Nix](https://nixos.org/download.html) with flakes enabled

### Setup

Allow direnv to automatically load the development environment:

```console
direnv allow
```

### Build

```console
make build
```

## TODO

- [ ] Implement the archive creation functions (zip and tar.gz)
- [ ] Add more detailed exclude patterns for each platform
- [ ] Implement proper error handling for various failure scenarios
- [ ] Add retry logic for interrupted operations
- [ ] Add tests
- [ ] Add CI/CD to build all platforms binaries
- [ ] Replace use of banaries with libs
  - [x] https://github.com/trevyn/librclone
    - [ ] Add progress to uploading
    - [x] macOS
    - [ ] Linux
    - [ ] Windows support
      - [ ] Replace 7z

## Configure project

```console
gh repo create --public backup-home
direnv allow
```

## Reasoning

Wanted to make that all work in rust, but as it appears there's no lib for
rclone, there's librclone, which you cna use to embed into rust binary. Still I
did not want to make things that complex.

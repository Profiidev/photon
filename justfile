build:
  cargo bootimage

qemu-debug:
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-photon/debug/bootimage-photon.bin

qemu-release:
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-photon/release/bootimage-photon.bin

run-debug: build qemu-debug

run-release: build qemu-release

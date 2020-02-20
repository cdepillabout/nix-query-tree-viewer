# nix-query-tree-viewer

[![Actions Status](https://github.com/cdepillabout/nix-query-tree-viewer/workflows/Test/badge.svg)](https://github.com/cdepillabout/nix-query-tree-viewer/actions)
[![crates.io](https://img.shields.io/crates/v/nix-query-tree-viewer.svg)](https://crates.io/crates/nix-query-tree-viewer)
[![BSD3 license](https://img.shields.io/badge/license-BSD3-blue.svg)](./LICENSE)

`nix-query-tree-viewer` is a convenient way to visualize the output of
the dependencies of a given path in the Nix store.

![image of nix-query-tree-viewer](./img/screenshot.png)

This is the same tree information that `nix-store --query --tree <PATH>` outputs,
but `nix-query-tree-viewer` makes it easier to understand.

## Usage

You can run `nix-query-tree-viewer` by passing it a path in the Nix store:

```console
$ nix-query-tree-viewer /nix/store/ghzg4kg0sjif58smj2lfm2bdvjwim85y-gcc-wrapper-7.4.0
```

## Why


```console
$ nix-build '<nixpkgs>' -A gcc
...
/nix/store/ghzg4kg0sjif58smj2lfm2bdvjwim85y-gcc-wrapper-7.4.0
```


```console
$ nix-store --query --tree /nix/store/ghzg4kg0sjif58smj2lfm2bdvjwim85y-gcc-wrapper-7.4.0
/nix/store/ghzg4kg0sjif58smj2lfm2bdvjwim85y-gcc-wrapper-7.4.0
+---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
+---/nix/store/cinw572b38aln37glr0zb8lxwrgaffl4-bash-4.4-p23
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/cinw572b38aln37glr0zb8lxwrgaffl4-bash-4.4-p23 [...]
+---/nix/store/hlnxw4k6931bachvg5sv0cyaissimswb-gcc-7.4.0-lib
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/hlnxw4k6931bachvg5sv0cyaissimswb-gcc-7.4.0-lib [...]
+---/nix/store/f5wl80zkrd3fc1jxsljmnpn7y02lz6v1-glibc-2.27-bin
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/f5wl80zkrd3fc1jxsljmnpn7y02lz6v1-glibc-2.27-bin [...]
+---/nix/store/sr4253np2gz2bpha4gn8gqlmiw604155-glibc-2.27-dev
|   +---/nix/store/5lyvydxv0w4f2s1ba84pjlbpvqkgn1ni-linux-headers-4.19.16
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/f5wl80zkrd3fc1jxsljmnpn7y02lz6v1-glibc-2.27-bin [...]
+---/nix/store/d4n93jn9fdq8fkmkm1q8f32lfagvibjk-gcc-7.4.0
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/hlnxw4k6931bachvg5sv0cyaissimswb-gcc-7.4.0-lib [...]
|   +---/nix/store/iiymx8j7nlar3gc23lfkcscvr61fng8s-zlib-1.2.11
|   |   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/sr4253np2gz2bpha4gn8gqlmiw604155-glibc-2.27-dev [...]
|   +---/nix/store/d4n93jn9fdq8fkmkm1q8f32lfagvibjk-gcc-7.4.0 [...]
+---/nix/store/d9s1kq1bnwqgxwcvv4zrc36ysnxg8gv7-coreutils-8.30
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/hql9ki8x230src24ljb9fad7rxxpzal0-attr-2.4.48
|   |   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   |   +---/nix/store/hql9ki8x230src24ljb9fad7rxxpzal0-attr-2.4.48 [...]
|   +---/nix/store/s83xl21h4qiz5yxlhr0n2bm1dl574mhw-acl-2.2.53
|   |   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   |   +---/nix/store/hql9ki8x230src24ljb9fad7rxxpzal0-attr-2.4.48 [...]
|   |   +---/nix/store/s83xl21h4qiz5yxlhr0n2bm1dl574mhw-acl-2.2.53 [...]
|   +---/nix/store/d9s1kq1bnwqgxwcvv4zrc36ysnxg8gv7-coreutils-8.30 [...]
+---/nix/store/mwx2860fvs3fq5dyzassvbkrkys63qsf-expand-response-params
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
+---/nix/store/rbpyfy6413aqpik9aj6p3a2syd1mda68-binutils-wrapper-2.31.1
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/0y7jmqnj48ikjh37n3dl9kqw9hnn68nq-binutils-2.31.1
|   |   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   |   +---/nix/store/iiymx8j7nlar3gc23lfkcscvr61fng8s-zlib-1.2.11 [...]
|   |   +---/nix/store/0y7jmqnj48ikjh37n3dl9kqw9hnn68nq-binutils-2.31.1 [...]
|   +---/nix/store/cinw572b38aln37glr0zb8lxwrgaffl4-bash-4.4-p23 [...]
|   +---/nix/store/d9s1kq1bnwqgxwcvv4zrc36ysnxg8gv7-coreutils-8.30 [...]
|   +---/nix/store/f5wl80zkrd3fc1jxsljmnpn7y02lz6v1-glibc-2.27-bin [...]
|   +---/nix/store/mwx2860fvs3fq5dyzassvbkrkys63qsf-expand-response-params [...]
|   +---/nix/store/sr4253np2gz2bpha4gn8gqlmiw604155-glibc-2.27-dev [...]
|   +---/nix/store/rbpyfy6413aqpik9aj6p3a2syd1mda68-binutils-wrapper-2.31.1 [...]
+---/nix/store/wnjv27b3j6jfdl0968xpcymlc7chpqil-gnugrep-3.3
|   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   +---/nix/store/0j1sc30kjf9b3j7j0sp68jns2v34apr0-pcre-8.42
|   |   +---/nix/store/681354n3k44r8z90m35hm8945vsp95h1-glibc-2.27 [...]
|   |   +---/nix/store/0j1sc30kjf9b3j7j0sp68jns2v34apr0-pcre-8.42 [...]
|   +---/nix/store/wnjv27b3j6jfdl0968xpcymlc7chpqil-gnugrep-3.3 [...]
+---/nix/store/ghzg4kg0sjif58smj2lfm2bdvjwim85y-gcc-wrapper-7.4.0 [...]
```

OpenGame
======
OpenGame (OG) is a gaming dependencies and ProtonGE (like protonup) installer for multi linux distro

<details>
<summary>Distributions</summary>

+ Fedora
+ Arch (need test)
+ Ubuntu
+ ElementaryOS
</details>

<details>
<summary>Dependencies</summary>

Install curl on your system to use installation command

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details>
<summary>Build</summary>

Use <a target="_blank" href="https://github.com/rust-embedded/cross">cross</a> to compile for
a release with target `x86_64-unknown-linux-gnu`<br>
You can use `cargo check` or `cargo run` for debug

```shell
cross build --target x86_64-unknown-linux-gnu --release
```
</details>

<details>
<summary>Installation</summary>

Install curl on your system to use installation command

```shell
curl -L https://raw.githubusercontent.com/Dragnansia/OpenGame/main/install.sh | sh
```
</details>

<details>
<summary>Remove</summary>

```shell
rm ~/bin/opengame
```
</details>

<details>
<summary>Other projects</summary>

+ <a target="_blank" href="https://github.com/Ahmed-Al-Balochi/LibreGaming">LibreGaming</a>
</details>

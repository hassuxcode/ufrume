<img width="3000" height="1000" alt="Twitter Header" src="https://github.com/user-attachments/assets/04e5058c-affe-4192-ba5b-6326574033f6" />

<br>
<br>

A multithreaded CLI tool to organize your music files into a folder structure defined by you.

<table>
<tr>
<td valign="top">

**BEFORE**
```
music/
├── DEEP.flac
├── The Storm.flac
├── Walk Slowly.flac
├── 2U.flac
├── Good Morning Vietnam.flac
├── Too Much (extended mix).flac
├── jungle_compilation_track01.flac
└── [500+ more randomly named files...]
```

</td>
<td valign="top">

**AFTER**
```
music/
├── Bad Computer/
│   └── 2020 - 2U/
│       └── 01 - 2U.flac
├── Example/
│   └── 2021 - DEEP/
│       └── 01 - DEEP.flac
├── TheFatRat/
│   └── 2019 - The Storm/
│       └── 01 - The Storm.flac
├── Ballpoint/
│   └── 2022 - Walk Slowly/
│       └── 01 - Walk Slowly.flac
├── Marc Benjamin/
│   └── 2023 - Too Much/
│       └── 01 - Too Much (extended mix).flac
├── Shotgun Willy/
│   └── 2020 - Good Morning Vietnam/
│       └── 01 - Good Morning Vietnam.flac
└── Compilations/
    └── Welcome to the Jungle- The Ultimate Jungle Cakes Drum & Bass Compilation/
        ├── 01 - DJ Deekline & Ed Solo feat. Top Cat - Bad Boys.flac
        ├── 02 - DJ Deekline & Ed Solo - No No No (Serial Killaz remix).flac
        └── 12 - Ricky Tuffy feat. Ras Mc Bean - Brighter Day.flac
```

</td>
</tr>
</table>

## Installation

The easiest way is to install it directly from [crates.io](https://crates.io/crates/ufrume):

```zsh
cargo install ufrume
```


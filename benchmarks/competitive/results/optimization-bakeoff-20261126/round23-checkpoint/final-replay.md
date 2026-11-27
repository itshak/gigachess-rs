# Supplemental exact final-output replay

CI valid; single-run medians, not included in strict 522.

| Variant | Platform | Full loop | Recompute | Hashed | Shakmaty | Recompute vs full | Recompute vs Shakmaty |
|---|---|---:|---:|---:|---:|---:|---:|
| Direct | combined-macos-15-native | 360539 | 300812 | 250445 | 222538 | 1.199x | 0.740x |
| Direct | combined-ubuntu-24.04-arm-native | 278242 | 198963 | 208552 | 170969 | 1.398x | 0.859x |
| Direct | combined-ubuntu-24.04-native | 317924 | 230823 | 255197 | 215384 | 1.377x | 0.933x |
| Direct | combined-ubuntu-24.04-pext | 326365 | 238489 | 243934 | 222035 | 1.368x | 0.931x |
| Direct | combined-ubuntu-24.04-portable | 309962 | 237452 | 260336 | 222741 | 1.305x | 0.938x |
| Direct | combined-windows-2025-native | 351268 | 256968 | 276185 | 249884 | 1.367x | 0.972x |
| Pair | combined-macos-15-native | 306494 | 264546 | 310086 | 247628 | 1.159x | 0.936x |
| Pair | combined-ubuntu-24.04-arm-native | 279847 | 201843 | 209883 | 169913 | 1.386x | 0.842x |
| Pair | combined-ubuntu-24.04-native | 153018 | 114285 | 111780 | 185609 | 1.339x | 1.624x |
| Pair | combined-ubuntu-24.04-pext | 344502 | 257324 | 267010 | 247788 | 1.339x | 0.963x |
| Pair | combined-ubuntu-24.04-portable | 315425 | 243380 | 262555 | 218434 | 1.296x | 0.898x |
| Pair | combined-windows-2025-native | 350600 | 255246 | 274532 | 260455 | 1.374x | 1.020x |

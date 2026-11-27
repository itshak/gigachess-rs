# Corrected local final-output batch comparison

| Variant | Median ns/18954 plies | Median per-run ratio to Shakmaty |
|---|---:|---:|
| replay | 333222 | 0.937x |
| direct | 308586 | 1.050x |
| pair | 323077 | 0.967x |
| pair-fixed | 320862 | 0.951x |

Fixed-pair CI wins two of six; not part of native strict score.

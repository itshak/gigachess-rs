| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.549x |
| materialize | 1.002x | 0.999x | 1.327x |
| perft_bulk_d4 | 0.999x | 0.995x | 1.389x |
| perft_full_d3 | 1.603x | 1.479x | 1.336x |
| fen_parse | 0.998x | 0.992x | 1.198x |
| fen_write_native_ep | 0.989x | 0.984x | 1.571x |
| snapshot | 0.994x | 0.991x | 0.944x |
| san_parse_all | 0.999x | 0.995x | 1.818x |
| san_render_native | 0.997x | 0.994x | 2.516x |
| apply_children_full | 0.998x | 0.983x | 1.168x |
| make_unmake_full | 0.996x | 0.977x | 1.551x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 1.000x | 0.999x | 1.504x |
| chess960_perft_d3 | 0.991x | 0.988x | 1.490x |
| polyglot_key | 0.994x | 0.982x | 25.891x |
| polyglot_recompute | 0.995x | 0.989x | 1.084x |
| repetition_key | 0.993x | 0.985x | 19.391x |
| corpus_count | 0.999x | 0.997x | 1.343x |
| corpus_materialize | 0.999x | 0.998x | 1.226x |
| corpus_uci_sorted | 1.006x | 0.999x | 1.463x |
| replay_trusted_native | 1.007x | 1.006x | 0.616x |
| replay_checked_native | 1.010x | 1.008x | 1.055x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.988x | 1.376x |
| materialize | 1.042x | 1.010x | 1.518x |
| perft_bulk_d4 | 1.031x | 1.008x | 1.039x |
| perft_full_d3 | 0.997x | 0.989x | 1.450x |
| fen_parse | 1.061x | 1.007x | 1.103x |
| fen_write_native_ep | 0.994x | 0.968x | 1.893x |
| snapshot | 1.004x | 0.989x | 1.157x |
| san_parse_all | 0.991x | 0.978x | 2.021x |
| san_render_native | 0.991x | 0.984x | 2.172x |
| apply_children_full | 0.980x | 0.956x | 1.255x |
| make_unmake_full | 0.992x | 0.917x | 1.135x |
| startpos | 0.891x | 0.891x | 1.117x |
| chess960_materialize | 0.996x | 0.990x | 1.449x |
| chess960_perft_d3 | 0.998x | 0.994x | 1.408x |
| polyglot_key | 0.991x | 0.982x | 39.769x |
| polyglot_recompute | 0.950x | 0.922x | 0.977x |
| repetition_key | 0.846x | 0.780x | 16.085x |
| corpus_count | 1.013x | 1.006x | 0.889x |
| corpus_materialize | 1.005x | 1.003x | 1.041x |
| corpus_uci_sorted | 0.997x | 0.992x | 1.545x |
| replay_trusted_native | 0.984x | 0.943x | 0.885x |
| replay_checked_native | 1.041x | 0.978x | 1.114x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.80x).

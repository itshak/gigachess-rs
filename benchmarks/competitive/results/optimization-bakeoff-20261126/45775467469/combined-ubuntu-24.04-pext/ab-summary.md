| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 1.000x | 1.443x |
| materialize | 1.005x | 1.001x | 1.516x |
| perft_bulk_d4 | 0.990x | 0.951x | 1.226x |
| perft_full_d3 | 1.509x | 1.386x | 1.357x |
| fen_parse | 0.964x | 0.944x | 1.152x |
| fen_write_native_ep | 1.013x | 1.002x | 1.228x |
| snapshot | 0.774x | 0.771x | 1.002x |
| san_parse_all | 0.975x | 0.967x | 1.827x |
| san_render_native | 0.984x | 0.979x | 2.070x |
| apply_children_full | 1.024x | 0.983x | 0.980x |
| make_unmake_full | 0.995x | 0.986x | 0.947x |
| startpos | 0.815x | 0.815x | 1.251x |
| chess960_materialize | 0.999x | 0.999x | 1.389x |
| chess960_perft_d3 | 1.007x | 1.005x | 1.301x |
| polyglot_key | 0.934x | 0.868x | 38.818x |
| polyglot_recompute | 0.999x | 0.995x | 1.053x |
| repetition_key | 0.931x | 0.868x | 19.462x |
| corpus_count | 0.998x | 0.992x | 1.391x |
| corpus_materialize | 1.007x | 1.002x | 1.116x |
| corpus_uci_sorted | 1.017x | 1.001x | 0.966x |
| replay_trusted_native | 1.009x | 1.006x | 0.616x |
| replay_checked_native | 0.960x | 0.932x | 0.934x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.88x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.992x | 1.278x |
| materialize | 0.996x | 0.989x | 1.718x |
| perft_bulk_d4 | 0.994x | 0.985x | 1.181x |
| perft_full_d3 | 1.001x | 0.995x | 1.328x |
| fen_parse | 1.004x | 0.974x | 1.134x |
| fen_write_native_ep | 0.988x | 0.961x | 1.839x |
| snapshot | 0.998x | 0.992x | 1.002x |
| san_parse_all | 1.006x | 0.983x | 1.885x |
| san_render_native | 1.016x | 1.002x | 2.145x |
| apply_children_full | 0.997x | 0.993x | 1.321x |
| make_unmake_full | 0.989x | 0.968x | 1.253x |
| startpos | 0.999x | 0.999x | 1.356x |
| chess960_materialize | 1.057x | 1.050x | 1.569x |
| chess960_perft_d3 | 0.997x | 0.994x | 1.492x |
| polyglot_key | 0.961x | 0.929x | 40.462x |
| polyglot_recompute | 1.004x | 0.988x | 0.989x |
| repetition_key | 0.901x | 0.830x | 19.185x |
| corpus_count | 1.004x | 0.996x | 1.181x |
| corpus_materialize | 1.001x | 0.998x | 1.146x |
| corpus_uci_sorted | 0.996x | 0.996x | 1.554x |
| replay_trusted_native | 0.958x | 0.932x | 0.839x |
| replay_checked_native | 0.983x | 0.970x | 1.076x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.79x).

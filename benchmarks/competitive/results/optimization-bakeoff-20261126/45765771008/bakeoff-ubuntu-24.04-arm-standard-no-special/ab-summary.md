| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.993x | 1.088x |
| materialize | 1.016x | 0.998x | 1.195x |
| perft_bulk_d4 | 1.026x | 1.011x | 0.903x |
| perft_full_d3 | 1.003x | 0.998x | 0.599x |
| fen_parse | 0.998x | 0.994x | 1.195x |
| fen_write_native_ep | 0.998x | 0.995x | 1.058x |
| snapshot | 0.980x | 0.964x | 0.693x |
| san_parse_all | 0.996x | 0.990x | 0.918x |
| san_render_native | 0.996x | 0.989x | 1.149x |
| apply_children_full | 0.997x | 0.991x | 0.654x |
| make_unmake_full | 1.003x | 0.994x | 0.902x |
| startpos | 1.000x | 1.000x | 0.783x |
| chess960_materialize | 1.002x | 0.999x | 1.113x |
| chess960_perft_d3 | 0.993x | 0.989x | 0.837x |
| polyglot_key | 1.008x | 1.005x | 23.616x |
| polyglot_recompute | 0.999x | 0.992x | 1.129x |
| repetition_key | 0.990x | 0.981x | 14.156x |
| corpus_count | 1.014x | 1.014x | 1.199x |
| corpus_materialize | 1.016x | 1.007x | 1.143x |
| corpus_uci_sorted | 0.996x | 0.992x | 0.565x |
| replay_trusted_native | 1.004x | 1.002x | 0.323x |
| replay_checked_native | 1.006x | 1.004x | 0.661x |

Cold first legal move list: 154.002 ms → 153.916 ms (1.00x).

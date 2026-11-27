| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 1.016x | 1.514x |
| materialize | 0.990x | 0.963x | 1.142x |
| perft_bulk_d4 | 1.041x | 1.010x | 1.394x |
| perft_full_d3 | 1.357x | 1.070x | 1.609x |
| fen_parse | 1.007x | 0.983x | 1.201x |
| fen_write_native_ep | 1.032x | 0.996x | 1.943x |
| snapshot | 1.005x | 0.998x | 0.661x |
| san_parse_all | 1.006x | 0.999x | 2.231x |
| san_render_native | 1.004x | 0.993x | 2.109x |
| apply_children_full | 1.008x | 0.983x | 1.284x |
| make_unmake_full | 1.004x | 0.991x | 1.210x |
| startpos | 1.001x | 1.001x | 1.201x |
| chess960_materialize | 0.983x | 0.979x | 1.558x |
| chess960_perft_d3 | 0.988x | 0.979x | 1.583x |
| polyglot_key | 0.998x | 0.991x | 20.509x |
| polyglot_recompute | 1.004x | 1.002x | 1.026x |
| repetition_key | 0.998x | 0.994x | 19.120x |
| corpus_count | 1.072x | 1.046x | 1.389x |
| corpus_materialize | 1.037x | 1.021x | 1.237x |
| corpus_uci_sorted | 0.982x | 0.972x | 1.679x |
| replay_trusted_native | 1.117x | 1.009x | 1.184x |
| replay_checked_native | 1.017x | 1.009x | 1.140x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.19x).

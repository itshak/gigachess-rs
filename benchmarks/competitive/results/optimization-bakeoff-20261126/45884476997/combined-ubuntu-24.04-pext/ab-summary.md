| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.985x | 1.570x |
| materialize | 1.050x | 1.021x | 1.637x |
| perft_bulk_d4 | 1.008x | 0.989x | 1.197x |
| perft_full_d3 | 1.155x | 1.047x | 1.736x |
| fen_parse | 0.939x | 0.826x | 0.987x |
| fen_write_native_ep | 1.008x | 1.002x | 2.155x |
| snapshot | 1.000x | 0.992x | 1.325x |
| san_parse_all | 1.039x | 1.011x | 1.920x |
| san_render_native | 1.001x | 0.994x | 2.165x |
| apply_children_full | 1.009x | 0.999x | 1.267x |
| make_unmake_full | 1.023x | 0.981x | 1.222x |
| startpos | 1.000x | 1.000x | 1.252x |
| chess960_materialize | 1.017x | 0.999x | 1.541x |
| chess960_perft_d3 | 1.014x | 1.003x | 1.494x |
| polyglot_key | 1.011x | 1.007x | 39.861x |
| polyglot_recompute | 1.079x | 1.043x | 1.079x |
| repetition_key | 0.808x | 0.693x | 19.216x |
| corpus_count | 0.951x | 0.903x | 1.077x |
| corpus_materialize | 1.016x | 1.008x | 1.079x |
| corpus_uci_sorted | 1.021x | 1.018x | 1.571x |
| replay_trusted_native | 0.893x | 0.880x | 0.806x |
| replay_checked_native | 0.840x | 0.798x | 0.940x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.20x).

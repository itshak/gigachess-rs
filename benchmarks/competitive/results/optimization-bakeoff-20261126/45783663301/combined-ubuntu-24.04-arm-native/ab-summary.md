| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.354x |
| materialize | 1.010x | 1.005x | 1.234x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.350x |
| perft_full_d3 | 0.994x | 0.950x | 1.046x |
| fen_parse | 0.993x | 0.978x | 1.230x |
| fen_write_native_ep | 0.995x | 0.991x | 1.034x |
| snapshot | 1.048x | 1.046x | 1.031x |
| san_parse_all | 1.004x | 0.996x | 1.820x |
| san_render_native | 1.000x | 0.988x | 2.556x |
| apply_children_full | 1.011x | 1.000x | 1.028x |
| make_unmake_full | 1.011x | 1.004x | 1.449x |
| startpos | 0.966x | 0.966x | 1.155x |
| chess960_materialize | 1.016x | 1.005x | 1.334x |
| chess960_perft_d3 | 1.006x | 0.996x | 1.322x |
| polyglot_key | 1.006x | 1.002x | 25.565x |
| polyglot_recompute | 0.998x | 0.997x | 1.074x |
| repetition_key | 1.007x | 1.000x | 18.842x |
| corpus_count | 0.998x | 0.997x | 1.499x |
| corpus_materialize | 1.010x | 1.009x | 1.214x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.461x |
| replay_trusted_native | 0.996x | 0.991x | 0.496x |
| replay_checked_native | 1.000x | 0.999x | 0.988x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.88x).

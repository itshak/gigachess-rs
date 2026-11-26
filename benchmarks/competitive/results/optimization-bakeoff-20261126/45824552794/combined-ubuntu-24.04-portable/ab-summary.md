| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.225x |
| materialize | 0.998x | 0.994x | 1.621x |
| perft_bulk_d4 | 1.000x | 0.998x | 1.063x |
| perft_full_d3 | 0.999x | 0.998x | 1.456x |
| fen_parse | 0.998x | 0.994x | 1.177x |
| fen_write_native_ep | 1.001x | 0.989x | 1.861x |
| snapshot | 0.999x | 0.995x | 1.000x |
| san_parse_all | 0.999x | 0.993x | 1.897x |
| san_render_native | 1.035x | 1.014x | 2.191x |
| apply_children_full | 1.000x | 0.996x | 1.309x |
| make_unmake_full | 0.998x | 0.995x | 1.230x |
| startpos | 1.000x | 1.000x | 1.287x |
| chess960_materialize | 1.004x | 1.001x | 1.368x |
| chess960_perft_d3 | 1.003x | 1.002x | 1.393x |
| polyglot_key | 1.002x | 1.000x | 43.398x |
| polyglot_recompute | 0.992x | 0.980x | 1.059x |
| repetition_key | 1.017x | 1.002x | 20.307x |
| corpus_count | 0.999x | 0.992x | 0.996x |
| corpus_materialize | 1.000x | 0.997x | 1.079x |
| corpus_uci_sorted | 1.013x | 1.011x | 1.516x |
| replay_trusted_native | 0.999x | 0.998x | 0.908x |
| replay_checked_native | 1.003x | 0.998x | 1.045x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.86x).

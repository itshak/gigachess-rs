| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.207x |
| materialize | 1.064x | 1.037x | 1.600x |
| perft_bulk_d4 | 0.999x | 0.993x | 1.061x |
| perft_full_d3 | 1.004x | 0.970x | 1.465x |
| fen_parse | 0.922x | 0.802x | 1.040x |
| fen_write_native_ep | 0.981x | 0.968x | 1.827x |
| snapshot | 0.995x | 0.980x | 0.996x |
| san_parse_all | 0.987x | 0.952x | 1.861x |
| san_render_native | 0.981x | 0.964x | 2.096x |
| apply_children_full | 0.991x | 0.984x | 1.260x |
| make_unmake_full | 0.995x | 0.984x | 1.218x |
| startpos | 1.006x | 1.006x | 1.441x |
| chess960_materialize | 1.002x | 0.983x | 1.381x |
| chess960_perft_d3 | 0.995x | 0.994x | 1.384x |
| polyglot_key | 1.001x | 1.000x | 42.640x |
| polyglot_recompute | 0.934x | 0.915x | 0.974x |
| repetition_key | 1.016x | 1.003x | 20.390x |
| corpus_count | 1.011x | 0.989x | 1.029x |
| corpus_materialize | 0.991x | 0.989x | 1.086x |
| corpus_uci_sorted | 1.002x | 0.993x | 1.533x |
| replay_trusted_native | 0.992x | 0.980x | 0.873x |
| replay_checked_native | 1.023x | 1.016x | 1.028x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.96x).

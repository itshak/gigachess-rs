| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.938x | 0.734x | 1.310x |
| materialize | 0.989x | 0.947x | 1.079x |
| perft_bulk_d4 | 0.968x | 0.885x | 1.329x |
| perft_full_d3 | 0.992x | 0.981x | 1.057x |
| fen_parse | 0.992x | 0.958x | 1.257x |
| fen_write_native_ep | 1.006x | 0.969x | 1.603x |
| snapshot | 0.969x | 0.853x | 1.065x |
| san_parse_all | 0.988x | 0.973x | 2.161x |
| san_render_native | 0.979x | 0.941x | 3.450x |
| apply_children_full | 0.781x | 0.686x | 0.836x |
| make_unmake_full | 1.001x | 0.983x | 1.235x |
| startpos | 0.998x | 0.998x | 1.081x |
| chess960_materialize | 1.008x | 1.004x | 1.220x |
| chess960_perft_d3 | 1.001x | 0.996x | 1.411x |
| polyglot_key | 1.001x | 1.000x | 20.389x |
| polyglot_recompute | 0.995x | 0.990x | 0.981x |
| repetition_key | 1.065x | 0.996x | 17.858x |
| corpus_count | 0.993x | 0.988x | 1.323x |
| corpus_materialize | 0.998x | 0.996x | 1.173x |
| corpus_uci_sorted | 1.005x | 1.002x | 1.519x |
| replay_trusted_native | 1.004x | 0.999x | 0.751x |
| replay_checked_native | 1.032x | 1.009x | 0.934x |

Cold first legal move list: 0.028 ms -> 0.029 ms (0.96x).

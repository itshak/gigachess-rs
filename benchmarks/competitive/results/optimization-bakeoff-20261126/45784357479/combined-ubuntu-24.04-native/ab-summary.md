| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.996x | 1.412x |
| materialize | 1.054x | 1.029x | 1.518x |
| perft_bulk_d4 | 0.997x | 0.988x | 1.208x |
| perft_full_d3 | 1.006x | 1.000x | 1.506x |
| fen_parse | 1.078x | 1.040x | 1.095x |
| fen_write_native_ep | 0.996x | 0.980x | 1.894x |
| snapshot | 1.000x | 0.992x | 1.000x |
| san_parse_all | 1.041x | 1.021x | 1.935x |
| san_render_native | 1.003x | 0.994x | 2.151x |
| apply_children_full | 0.989x | 0.971x | 1.069x |
| make_unmake_full | 0.989x | 0.960x | 1.138x |
| startpos | 0.799x | 0.799x | 1.003x |
| chess960_materialize | 1.135x | 1.128x | 1.468x |
| chess960_perft_d3 | 0.997x | 0.993x | 1.385x |
| polyglot_key | 0.983x | 0.966x | 39.175x |
| polyglot_recompute | 0.943x | 0.885x | 0.910x |
| repetition_key | 1.172x | 1.021x | 22.156x |
| corpus_count | 1.000x | 0.996x | 1.333x |
| corpus_materialize | 1.012x | 1.010x | 1.081x |
| corpus_uci_sorted | 0.992x | 0.988x | 1.562x |
| replay_trusted_native | 1.003x | 0.999x | 0.698x |
| replay_checked_native | 1.012x | 1.012x | 1.114x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.98x).

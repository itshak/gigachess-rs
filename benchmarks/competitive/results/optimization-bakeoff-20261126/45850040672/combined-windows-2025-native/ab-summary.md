| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.990x | 1.544x |
| materialize | 1.009x | 0.993x | 1.369x |
| perft_bulk_d4 | 1.004x | 0.996x | 1.170x |
| perft_full_d3 | 1.000x | 0.995x | 1.379x |
| fen_parse | 1.034x | 0.978x | 1.276x |
| fen_write_native_ep | 1.021x | 0.989x | 1.647x |
| snapshot | 1.132x | 1.111x | 1.114x |
| san_parse_all | 0.998x | 0.980x | 1.762x |
| san_render_native | 1.055x | 1.009x | 4.218x |
| apply_children_full | 1.029x | 1.011x | 1.339x |
| make_unmake_full | 1.039x | 0.986x | 1.219x |
| startpos | 1.006x | 1.006x | 1.008x |
| chess960_materialize | 1.004x | 1.001x | 1.349x |
| chess960_perft_d3 | 1.047x | 1.010x | 1.504x |
| polyglot_key | 0.976x | 0.951x | 39.608x |
| polyglot_recompute | 1.035x | 1.011x | 1.003x |
| repetition_key | 0.987x | 0.841x | 26.638x |
| corpus_count | 1.098x | 1.071x | 1.001x |
| corpus_materialize | 0.990x | 0.977x | 1.067x |
| corpus_uci_sorted | 1.021x | 1.018x | 1.550x |
| replay_trusted_native | 0.984x | 0.968x | 0.855x |
| replay_checked_native | 1.156x | 1.140x | 1.272x |

Cold first legal move list: 0.021 ms -> 0.018 ms (1.19x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 1.001x | 1.228x |
| materialize | 1.016x | 1.007x | 1.288x |
| perft_bulk_d4 | 0.996x | 0.987x | 1.170x |
| perft_full_d3 | 0.998x | 0.934x | 1.309x |
| fen_parse | 1.127x | 1.079x | 1.254x |
| fen_write_native_ep | 1.014x | 1.001x | 1.665x |
| snapshot | 0.999x | 0.997x | 1.017x |
| san_parse_all | 1.002x | 0.996x | 1.988x |
| san_render_native | 0.977x | 0.913x | 2.173x |
| apply_children_full | 1.036x | 1.008x | 1.361x |
| make_unmake_full | 1.032x | 1.012x | 1.432x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 1.071x | 1.028x | 1.339x |
| chess960_perft_d3 | 0.988x | 0.977x | 1.424x |
| polyglot_key | 0.961x | 0.921x | 26.494x |
| polyglot_recompute | 1.023x | 1.023x | 1.034x |
| repetition_key | 1.065x | 1.039x | 20.584x |
| corpus_count | 0.999x | 0.988x | 1.185x |
| corpus_materialize | 1.019x | 1.015x | 1.170x |
| corpus_uci_sorted | 1.010x | 0.999x | 1.718x |
| replay_trusted_native | 1.029x | 1.009x | 0.935x |
| replay_checked_native | 1.021x | 1.007x | 1.097x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.16x).

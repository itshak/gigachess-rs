| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.990x | 1.363x |
| materialize | 1.023x | 1.004x | 1.405x |
| perft_bulk_d4 | 0.997x | 0.988x | 1.013x |
| perft_full_d3 | 0.998x | 0.997x | 1.481x |
| fen_parse | 1.086x | 1.035x | 1.076x |
| fen_write_native_ep | 0.943x | 0.906x | 2.009x |
| snapshot | 1.000x | 0.998x | 1.159x |
| san_parse_all | 0.969x | 0.935x | 1.767x |
| san_render_native | 0.991x | 0.978x | 2.108x |
| apply_children_full | 1.001x | 0.990x | 1.295x |
| make_unmake_full | 0.998x | 0.984x | 1.171x |
| startpos | 1.000x | 1.000x | 1.120x |
| chess960_materialize | 1.000x | 1.000x | 1.252x |
| chess960_perft_d3 | 1.029x | 1.001x | 1.392x |
| polyglot_key | 0.983x | 0.933x | 38.410x |
| polyglot_recompute | 0.975x | 0.963x | 0.970x |
| repetition_key | 0.858x | 0.795x | 23.768x |
| corpus_count | 1.013x | 1.003x | 0.837x |
| corpus_materialize | 1.004x | 0.999x | 1.027x |
| corpus_uci_sorted | 1.008x | 1.000x | 1.554x |
| replay_trusted_native | 0.990x | 0.973x | 0.882x |
| replay_checked_native | 0.996x | 0.996x | 1.118x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.90x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.992x | 1.175x |
| materialize | 0.957x | 0.886x | 1.293x |
| perft_bulk_d4 | 1.090x | 1.066x | 0.947x |
| perft_full_d3 | 1.000x | 0.997x | 0.610x |
| fen_parse | 0.919x | 0.809x | 0.994x |
| fen_write_native_ep | 1.010x | 0.971x | 1.346x |
| snapshot | 1.185x | 1.183x | 0.625x |
| san_parse_all | 0.997x | 0.994x | 0.914x |
| san_render_native | 1.008x | 0.991x | 1.101x |
| apply_children_full | 1.029x | 1.023x | 0.681x |
| make_unmake_full | 0.998x | 0.988x | 0.758x |
| startpos | 0.949x | 0.949x | 0.591x |
| chess960_materialize | 1.049x | 1.043x | 1.226x |
| chess960_perft_d3 | 1.080x | 1.037x | 1.033x |
| polyglot_key | 1.121x | 1.001x | 40.048x |
| polyglot_recompute | 1.006x | 1.004x | 1.023x |
| repetition_key | 1.047x | 1.003x | 15.016x |
| corpus_count | 0.998x | 0.996x | 1.145x |
| corpus_materialize | 1.001x | 1.000x | 1.069x |
| corpus_uci_sorted | 0.967x | 0.965x | 0.612x |
| replay_trusted_native | 0.999x | 0.999x | 0.533x |
| replay_checked_native | 0.996x | 0.987x | 0.735x |

Cold first legal move list: 146.095 ms → 132.526 ms (1.10x).

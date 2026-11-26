| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.354x |
| materialize | 0.993x | 0.988x | 1.220x |
| perft_bulk_d4 | 1.003x | 1.000x | 1.351x |
| perft_full_d3 | 1.000x | 0.988x | 1.050x |
| fen_parse | 1.003x | 0.997x | 1.240x |
| fen_write_native_ep | 0.998x | 0.991x | 1.032x |
| snapshot | 0.955x | 0.954x | 0.895x |
| san_parse_all | 0.992x | 0.990x | 1.815x |
| san_render_native | 1.001x | 0.991x | 2.518x |
| apply_children_full | 0.975x | 0.938x | 0.998x |
| make_unmake_full | 1.045x | 1.026x | 1.488x |
| startpos | 0.963x | 0.963x | 1.181x |
| chess960_materialize | 0.999x | 0.998x | 1.323x |
| chess960_perft_d3 | 1.001x | 0.999x | 1.323x |
| polyglot_key | 0.973x | 0.954x | 24.577x |
| polyglot_recompute | 0.994x | 0.991x | 1.077x |
| repetition_key | 0.979x | 0.957x | 18.714x |
| corpus_count | 0.996x | 0.994x | 1.490x |
| corpus_materialize | 1.002x | 1.000x | 1.203x |
| corpus_uci_sorted | 0.996x | 0.995x | 1.487x |
| replay_trusted_native | 1.033x | 1.028x | 0.521x |
| replay_checked_native | 1.049x | 1.036x | 1.034x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.07x).

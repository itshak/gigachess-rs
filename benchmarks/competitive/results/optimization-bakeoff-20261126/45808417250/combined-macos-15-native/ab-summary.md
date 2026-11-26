| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.072x | 0.927x | 1.307x |
| materialize | 1.135x | 1.060x | 1.034x |
| perft_bulk_d4 | 1.059x | 0.913x | 1.288x |
| perft_full_d3 | 1.081x | 0.862x | 1.355x |
| fen_parse | 1.177x | 0.986x | 1.296x |
| fen_write_native_ep | 1.061x | 0.882x | 1.824x |
| snapshot | 0.983x | 0.777x | 0.931x |
| san_parse_all | 1.084x | 0.944x | 1.813x |
| san_render_native | 1.147x | 0.906x | 2.743x |
| apply_children_full | 1.036x | 0.902x | 0.950x |
| make_unmake_full | 0.912x | 0.674x | 1.043x |
| startpos | 1.029x | 1.029x | 1.327x |
| chess960_materialize | 0.939x | 0.894x | 1.709x |
| chess960_perft_d3 | 0.800x | 0.738x | 1.542x |
| polyglot_key | 0.987x | 0.984x | 38.161x |
| polyglot_recompute | 0.899x | 0.855x | 0.877x |
| repetition_key | 0.946x | 0.744x | 16.042x |
| corpus_count | 1.019x | 0.912x | 1.390x |
| corpus_materialize | 1.029x | 0.938x | 1.183x |
| corpus_uci_sorted | 0.961x | 0.880x | 1.546x |
| replay_trusted_native | 0.888x | 0.702x | 0.525x |
| replay_checked_native | 0.894x | 0.806x | 0.942x |

Cold first legal move list: 0.016 ms -> 0.014 ms (1.15x).

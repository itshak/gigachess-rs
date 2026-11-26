| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.131x | 1.028x | 1.223x |
| materialize | 1.020x | 1.002x | 1.190x |
| perft_bulk_d4 | 1.100x | 1.068x | 0.993x |
| perft_full_d3 | 1.037x | 1.029x | 0.645x |
| fen_parse | 1.008x | 1.001x | 1.205x |
| fen_write_native_ep | 0.998x | 0.981x | 1.040x |
| snapshot | 0.973x | 0.963x | 0.666x |
| san_parse_all | 1.036x | 1.019x | 0.956x |
| san_render_native | 1.051x | 1.045x | 1.208x |
| apply_children_full | 1.077x | 1.067x | 0.698x |
| make_unmake_full | 1.046x | 1.042x | 0.939x |
| startpos | 1.102x | 1.102x | 0.786x |
| chess960_materialize | 1.049x | 1.042x | 1.163x |
| chess960_perft_d3 | 1.086x | 1.083x | 0.914x |
| polyglot_key | 1.024x | 0.998x | 23.307x |
| polyglot_recompute | 1.004x | 1.000x | 1.137x |
| repetition_key | 1.019x | 1.016x | 14.256x |
| corpus_count | 1.138x | 1.097x | 1.353x |
| corpus_materialize | 1.048x | 1.042x | 1.166x |
| corpus_uci_sorted | 1.011x | 1.009x | 0.576x |
| replay_trusted_native | 1.058x | 1.058x | 0.341x |
| replay_checked_native | 1.061x | 1.052x | 0.701x |

Cold first legal move list: 153.938 ms → 0.010 ms (15656.87x).

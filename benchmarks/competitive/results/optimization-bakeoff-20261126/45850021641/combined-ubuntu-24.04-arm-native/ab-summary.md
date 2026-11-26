| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.552x |
| materialize | 0.997x | 0.981x | 1.445x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.520x |
| perft_full_d3 | 1.000x | 0.999x | 1.218x |
| fen_parse | 1.000x | 0.984x | 1.091x |
| fen_write_native_ep | 0.996x | 0.977x | 1.597x |
| snapshot | 0.998x | 0.993x | 0.978x |
| san_parse_all | 0.991x | 0.958x | 1.996x |
| san_render_native | 1.001x | 1.000x | 2.060x |
| apply_children_full | 1.006x | 0.993x | 1.026x |
| make_unmake_full | 1.020x | 0.986x | 1.648x |
| startpos | 1.000x | 1.000x | 1.338x |
| chess960_materialize | 0.998x | 0.997x | 1.501x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.714x |
| polyglot_key | 1.001x | 1.001x | 39.375x |
| polyglot_recompute | 1.001x | 0.999x | 1.050x |
| repetition_key | 1.005x | 1.002x | 30.361x |
| corpus_count | 1.000x | 0.999x | 1.420x |
| corpus_materialize | 1.004x | 0.999x | 1.310x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.550x |
| replay_trusted_native | 0.957x | 0.954x | 0.650x |
| replay_checked_native | 1.024x | 1.023x | 1.211x |

Cold first legal move list: 0.005 ms -> 0.004 ms (1.37x).

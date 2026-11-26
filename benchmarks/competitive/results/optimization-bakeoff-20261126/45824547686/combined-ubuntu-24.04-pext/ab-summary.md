| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.968x | 0.907x | 1.383x |
| materialize | 1.004x | 0.997x | 1.548x |
| perft_bulk_d4 | 0.999x | 0.997x | 1.055x |
| perft_full_d3 | 1.012x | 0.996x | 1.523x |
| fen_parse | 0.997x | 0.984x | 1.040x |
| fen_write_native_ep | 0.999x | 0.994x | 1.908x |
| snapshot | 1.000x | 0.996x | 1.329x |
| san_parse_all | 1.005x | 0.994x | 1.910x |
| san_render_native | 1.020x | 1.016x | 2.138x |
| apply_children_full | 0.997x | 0.988x | 1.208x |
| make_unmake_full | 0.999x | 0.978x | 1.167x |
| startpos | 1.111x | 1.111x | 1.247x |
| chess960_materialize | 0.988x | 0.949x | 1.380x |
| chess960_perft_d3 | 0.999x | 0.997x | 1.384x |
| polyglot_key | 0.935x | 0.876x | 38.243x |
| polyglot_recompute | 0.931x | 0.921x | 0.987x |
| repetition_key | 0.892x | 0.785x | 16.524x |
| corpus_count | 0.958x | 0.938x | 0.922x |
| corpus_materialize | 1.000x | 0.999x | 1.058x |
| corpus_uci_sorted | 1.018x | 1.016x | 1.562x |
| replay_trusted_native | 1.014x | 1.009x | 0.906x |
| replay_checked_native | 1.001x | 0.995x | 1.096x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).

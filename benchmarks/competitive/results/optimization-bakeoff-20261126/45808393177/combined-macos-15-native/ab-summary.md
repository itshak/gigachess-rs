| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.151x | 0.969x | 1.266x |
| materialize | 1.320x | 1.114x | 1.136x |
| perft_bulk_d4 | 0.926x | 0.576x | 1.137x |
| perft_full_d3 | 1.033x | 0.787x | 1.275x |
| fen_parse | 1.083x | 0.917x | 1.222x |
| fen_write_native_ep | 1.021x | 0.845x | 1.795x |
| snapshot | 1.075x | 1.003x | 0.993x |
| san_parse_all | 0.954x | 0.789x | 1.683x |
| san_render_native | 1.147x | 0.926x | 2.605x |
| apply_children_full | 1.308x | 1.170x | 1.236x |
| make_unmake_full | 1.222x | 1.106x | 1.417x |
| startpos | 1.011x | 1.011x | 1.221x |
| chess960_materialize | 1.115x | 1.069x | 1.384x |
| chess960_perft_d3 | 0.844x | 0.784x | 1.190x |
| polyglot_key | 1.100x | 1.091x | 38.131x |
| polyglot_recompute | 1.119x | 1.051x | 1.004x |
| repetition_key | 1.108x | 1.071x | 19.966x |
| corpus_count | 0.909x | 0.717x | 0.918x |
| corpus_materialize | 0.997x | 0.883x | 1.123x |
| corpus_uci_sorted | 1.015x | 0.987x | 1.489x |
| replay_trusted_native | 1.336x | 1.264x | 0.705x |
| replay_checked_native | 1.127x | 1.077x | 0.983x |

Cold first legal move list: 0.015 ms -> 0.006 ms (2.65x).

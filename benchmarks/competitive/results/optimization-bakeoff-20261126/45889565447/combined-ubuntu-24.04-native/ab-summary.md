| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.975x | 0.917x | 1.458x |
| materialize | 0.965x | 0.926x | 1.532x |
| perft_bulk_d4 | 0.983x | 0.920x | 1.143x |
| perft_full_d3 | 1.249x | 1.081x | 1.660x |
| fen_parse | 0.999x | 0.993x | 1.175x |
| fen_write_native_ep | 0.950x | 0.889x | 1.967x |
| snapshot | 1.002x | 0.989x | 1.000x |
| san_parse_all | 1.002x | 0.998x | 1.885x |
| san_render_native | 0.961x | 0.915x | 2.059x |
| apply_children_full | 1.000x | 0.987x | 1.353x |
| make_unmake_full | 0.997x | 0.965x | 1.192x |
| startpos | 0.802x | 0.802x | 0.993x |
| chess960_materialize | 0.909x | 0.897x | 1.383x |
| chess960_perft_d3 | 0.981x | 0.976x | 1.479x |
| polyglot_key | 1.008x | 0.963x | 40.320x |
| polyglot_recompute | 1.016x | 1.009x | 1.056x |
| repetition_key | 1.142x | 1.010x | 23.217x |
| corpus_count | 0.794x | 0.681x | 0.866x |
| corpus_materialize | 0.990x | 0.984x | 1.045x |
| corpus_uci_sorted | 0.970x | 0.938x | 1.489x |
| replay_trusted_native | 0.999x | 0.986x | 0.849x |
| replay_checked_native | 1.024x | 1.023x | 1.163x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.94x).

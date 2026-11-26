| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.380x |
| materialize | 1.001x | 0.995x | 1.244x |
| perft_bulk_d4 | 0.995x | 0.981x | 1.229x |
| perft_full_d3 | 1.011x | 0.993x | 1.079x |
| fen_parse | 1.006x | 1.001x | 1.246x |
| fen_write_native_ep | 1.003x | 0.999x | 1.601x |
| snapshot | 1.018x | 1.016x | 0.939x |
| san_parse_all | 0.998x | 0.988x | 1.817x |
| san_render_native | 1.018x | 1.008x | 2.598x |
| apply_children_full | 1.001x | 0.964x | 1.290x |
| make_unmake_full | 1.018x | 0.993x | 1.537x |
| startpos | 1.000x | 1.000x | 1.276x |
| chess960_materialize | 0.987x | 0.978x | 1.364x |
| chess960_perft_d3 | 0.998x | 0.995x | 1.341x |
| polyglot_key | 1.020x | 0.995x | 26.002x |
| polyglot_recompute | 1.055x | 1.037x | 1.125x |
| repetition_key | 1.005x | 1.004x | 18.414x |
| corpus_count | 0.987x | 0.984x | 1.189x |
| corpus_materialize | 0.998x | 0.991x | 1.153x |
| corpus_uci_sorted | 0.999x | 0.997x | 1.451x |
| replay_trusted_native | 0.981x | 0.970x | 0.597x |
| replay_checked_native | 0.990x | 0.987x | 1.048x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.22x).

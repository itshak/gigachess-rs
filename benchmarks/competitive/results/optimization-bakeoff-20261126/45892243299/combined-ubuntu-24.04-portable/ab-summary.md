| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.988x | 1.288x |
| materialize | 0.984x | 0.928x | 1.337x |
| perft_bulk_d4 | 0.960x | 0.899x | 1.232x |
| perft_full_d3 | 0.979x | 0.938x | 1.263x |
| fen_parse | 1.010x | 0.899x | 1.107x |
| fen_write_native_ep | 0.980x | 0.946x | 1.692x |
| snapshot | 1.000x | 0.998x | 0.978x |
| san_parse_all | 0.996x | 0.974x | 2.134x |
| san_render_native | 1.028x | 0.988x | 2.291x |
| apply_children_full | 0.999x | 0.968x | 1.417x |
| make_unmake_full | 1.000x | 0.978x | 1.363x |
| startpos | 0.999x | 0.999x | 1.335x |
| chess960_materialize | 0.926x | 0.909x | 1.513x |
| chess960_perft_d3 | 0.983x | 0.974x | 1.559x |
| polyglot_key | 0.995x | 0.991x | 22.081x |
| polyglot_recompute | 0.985x | 0.861x | 0.984x |
| repetition_key | 0.998x | 0.996x | 21.110x |
| corpus_count | 0.997x | 0.990x | 1.228x |
| corpus_materialize | 0.971x | 0.948x | 1.316x |
| corpus_uci_sorted | 1.011x | 1.002x | 1.595x |
| replay_trusted_native | 1.007x | 1.007x | 1.084x |
| replay_checked_native | 1.005x | 1.000x | 1.165x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.04x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.969x | 0.910x | 1.398x |
| materialize | 0.995x | 0.964x | 1.148x |
| perft_bulk_d4 | 0.954x | 0.907x | 1.266x |
| perft_full_d3 | 0.982x | 0.937x | 0.969x |
| fen_parse | 1.120x | 1.098x | 1.235x |
| fen_write_native_ep | 1.002x | 0.993x | 1.861x |
| snapshot | 3.074x | 3.011x | 1.110x |
| san_parse_all | 0.984x | 0.968x | 2.180x |
| san_render_native | 0.980x | 0.967x | 2.124x |
| apply_children_full | 0.965x | 0.947x | 1.033x |
| make_unmake_full | 0.999x | 0.982x | 1.202x |
| startpos | 1.066x | 1.066x | 1.147x |
| chess960_materialize | 1.028x | 1.000x | 1.535x |
| chess960_perft_d3 | 0.980x | 0.958x | 1.459x |
| polyglot_key | 0.993x | 0.987x | 20.402x |
| polyglot_recompute | 0.960x | 0.952x | 0.978x |
| repetition_key | 1.019x | 1.006x | 17.988x |
| corpus_count | 0.859x | 0.770x | 1.251x |
| corpus_materialize | 0.980x | 0.961x | 1.239x |
| corpus_uci_sorted | 1.001x | 0.990x | 1.654x |
| replay_trusted_native | 1.031x | 0.995x | 0.782x |
| replay_checked_native | 1.016x | 1.012x | 0.957x |

Cold first legal move list: 0.012 ms -> 0.006 ms (2.01x).

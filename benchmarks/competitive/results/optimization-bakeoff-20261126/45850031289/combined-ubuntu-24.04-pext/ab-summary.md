| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 1.004x | 1.560x |
| materialize | 1.016x | 1.003x | 1.639x |
| perft_bulk_d4 | 1.015x | 1.000x | 1.192x |
| perft_full_d3 | 1.004x | 0.999x | 1.344x |
| fen_parse | 1.004x | 0.976x | 1.134x |
| fen_write_native_ep | 0.987x | 0.957x | 1.860x |
| snapshot | 1.003x | 0.999x | 1.154x |
| san_parse_all | 1.072x | 1.051x | 1.836x |
| san_render_native | 1.006x | 1.000x | 2.123x |
| apply_children_full | 1.030x | 1.018x | 1.333x |
| make_unmake_full | 0.991x | 0.958x | 1.155x |
| startpos | 1.013x | 1.013x | 1.130x |
| chess960_materialize | 0.945x | 0.941x | 1.448x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.462x |
| polyglot_key | 0.927x | 0.854x | 36.492x |
| polyglot_recompute | 1.054x | 1.034x | 1.029x |
| repetition_key | 0.818x | 0.696x | 19.008x |
| corpus_count | 1.009x | 1.004x | 1.054x |
| corpus_materialize | 1.002x | 1.000x | 1.093x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.566x |
| replay_trusted_native | 0.999x | 0.976x | 0.884x |
| replay_checked_native | 1.112x | 1.076x | 1.177x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).

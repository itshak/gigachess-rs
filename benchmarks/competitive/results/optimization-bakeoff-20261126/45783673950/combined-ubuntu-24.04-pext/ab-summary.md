| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.995x | 1.484x |
| materialize | 0.941x | 0.901x | 1.482x |
| perft_bulk_d4 | 1.007x | 0.999x | 1.266x |
| perft_full_d3 | 0.725x | 0.677x | 1.068x |
| fen_parse | 1.037x | 1.000x | 1.129x |
| fen_write_native_ep | 1.013x | 0.996x | 1.321x |
| snapshot | 0.927x | 0.923x | 1.064x |
| san_parse_all | 0.945x | 0.916x | 1.774x |
| san_render_native | 1.004x | 0.998x | 2.116x |
| apply_children_full | 1.004x | 0.998x | 1.068x |
| make_unmake_full | 0.997x | 0.973x | 1.121x |
| startpos | 0.997x | 0.997x | 1.110x |
| chess960_materialize | 0.979x | 0.978x | 1.341x |
| chess960_perft_d3 | 0.990x | 0.988x | 1.388x |
| polyglot_key | 1.051x | 0.999x | 39.169x |
| polyglot_recompute | 0.998x | 0.997x | 1.013x |
| repetition_key | 0.982x | 0.981x | 21.533x |
| corpus_count | 0.989x | 0.989x | 1.403x |
| corpus_materialize | 1.012x | 1.008x | 1.126x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.562x |
| replay_trusted_native | 0.991x | 0.987x | 0.699x |
| replay_checked_native | 0.995x | 0.994x | 1.057x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.83x).

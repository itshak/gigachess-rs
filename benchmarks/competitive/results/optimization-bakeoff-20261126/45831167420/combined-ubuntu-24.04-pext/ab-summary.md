| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.109x | 0.933x | 1.531x |
| materialize | 1.050x | 0.971x | 1.421x |
| perft_bulk_d4 | 1.127x | 0.991x | 1.216x |
| perft_full_d3 | 1.005x | 0.960x | 1.292x |
| fen_parse | 0.925x | 0.897x | 1.125x |
| fen_write_native_ep | 1.014x | 0.994x | 1.978x |
| snapshot | 0.991x | 0.945x | 1.122x |
| san_parse_all | 1.012x | 0.992x | 1.953x |
| san_render_native | 0.996x | 0.989x | 2.454x |
| apply_children_full | 1.007x | 0.995x | 1.281x |
| make_unmake_full | 1.011x | 0.983x | 1.299x |
| startpos | 0.896x | 0.896x | 1.001x |
| chess960_materialize | 1.080x | 1.046x | 1.450x |
| chess960_perft_d3 | 1.044x | 1.013x | 1.379x |
| polyglot_key | 1.045x | 1.002x | 31.556x |
| polyglot_recompute | 0.956x | 0.933x | 1.050x |
| repetition_key | 1.022x | 0.999x | 20.267x |
| corpus_count | 1.113x | 1.060x | 1.062x |
| corpus_materialize | 1.061x | 1.044x | 1.153x |
| corpus_uci_sorted | 1.022x | 1.021x | 1.485x |
| replay_trusted_native | 1.006x | 1.001x | 0.855x |
| replay_checked_native | 1.007x | 1.003x | 1.067x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).

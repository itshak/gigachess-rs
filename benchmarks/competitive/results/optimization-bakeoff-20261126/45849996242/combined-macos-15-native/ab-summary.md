| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.985x | 1.545x |
| materialize | 0.971x | 0.893x | 1.063x |
| perft_bulk_d4 | 1.004x | 0.895x | 1.278x |
| perft_full_d3 | 1.023x | 0.909x | 1.329x |
| fen_parse | 1.054x | 0.985x | 1.296x |
| fen_write_native_ep | 0.943x | 0.685x | 1.738x |
| snapshot | 1.010x | 0.942x | 1.004x |
| san_parse_all | 1.002x | 0.960x | 1.713x |
| san_render_native | 1.007x | 0.969x | 2.736x |
| apply_children_full | 0.926x | 0.621x | 1.058x |
| make_unmake_full | 0.973x | 0.918x | 1.270x |
| startpos | 1.026x | 1.026x | 1.332x |
| chess960_materialize | 0.889x | 0.797x | 1.612x |
| chess960_perft_d3 | 0.937x | 0.894x | 1.423x |
| polyglot_key | 0.986x | 0.977x | 39.015x |
| polyglot_recompute | 1.014x | 0.991x | 1.043x |
| repetition_key | 0.963x | 0.961x | 19.877x |
| corpus_count | 0.970x | 0.901x | 1.122x |
| corpus_materialize | 1.036x | 0.994x | 1.156x |
| corpus_uci_sorted | 0.969x | 0.936x | 1.367x |
| replay_trusted_native | 1.000x | 0.985x | 0.650x |
| replay_checked_native | 1.056x | 1.049x | 1.157x |

Cold first legal move list: 0.003 ms -> 0.004 ms (0.71x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.984x | 1.341x |
| materialize | 0.945x | 0.900x | 1.099x |
| perft_bulk_d4 | 0.994x | 0.958x | 1.254x |
| perft_full_d3 | 0.926x | 0.890x | 0.887x |
| fen_parse | 0.913x | 0.885x | 0.994x |
| fen_write_native_ep | 1.002x | 0.969x | 1.490x |
| snapshot | 1.005x | 0.998x | 1.122x |
| san_parse_all | 0.994x | 0.964x | 1.848x |
| san_render_native | 1.045x | 0.990x | 2.313x |
| apply_children_full | 1.573x | 1.487x | 1.669x |
| make_unmake_full | 1.049x | 1.020x | 1.269x |
| startpos | 1.001x | 1.001x | 1.250x |
| chess960_materialize | 0.990x | 0.974x | 1.308x |
| chess960_perft_d3 | 1.021x | 1.005x | 1.441x |
| polyglot_key | 0.990x | 0.970x | 36.145x |
| polyglot_recompute | 0.999x | 0.996x | 0.981x |
| repetition_key | 1.022x | 1.022x | 28.533x |
| corpus_count | 1.007x | 0.998x | 1.294x |
| corpus_materialize | 0.945x | 0.901x | 1.241x |
| corpus_uci_sorted | 0.982x | 0.982x | 1.646x |
| replay_trusted_native | 0.991x | 0.985x | 0.652x |
| replay_checked_native | 0.994x | 0.982x | 0.876x |

Cold first legal move list: 0.012 ms -> 0.015 ms (0.84x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.030x | 0.938x | 1.324x |
| materialize | 1.002x | 0.954x | 1.351x |
| perft_bulk_d4 | 1.066x | 1.049x | 1.397x |
| perft_full_d3 | 1.185x | 1.035x | 1.474x |
| fen_parse | 0.950x | 0.919x | 1.266x |
| fen_write_native_ep | 1.039x | 0.995x | 1.665x |
| snapshot | 1.002x | 1.001x | 1.031x |
| san_parse_all | 0.997x | 0.982x | 1.990x |
| san_render_native | 1.027x | 0.996x | 2.273x |
| apply_children_full | 1.002x | 0.990x | 1.413x |
| make_unmake_full | 1.004x | 0.985x | 1.436x |
| startpos | 1.000x | 1.000x | 1.331x |
| chess960_materialize | 0.998x | 0.984x | 1.455x |
| chess960_perft_d3 | 1.026x | 1.002x | 1.599x |
| polyglot_key | 0.990x | 0.984x | 23.545x |
| polyglot_recompute | 1.014x | 1.009x | 1.022x |
| repetition_key | 1.016x | 1.007x | 18.178x |
| corpus_count | 1.096x | 1.083x | 1.409x |
| corpus_materialize | 1.001x | 0.994x | 1.172x |
| corpus_uci_sorted | 1.022x | 1.021x | 1.831x |
| replay_trusted_native | 0.998x | 0.998x | 0.890x |
| replay_checked_native | 0.970x | 0.956x | 1.099x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.03x).

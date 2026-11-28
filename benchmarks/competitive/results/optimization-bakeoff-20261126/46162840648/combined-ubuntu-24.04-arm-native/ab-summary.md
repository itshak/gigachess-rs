| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.987x | 1.571x |
| materialize | 1.013x | 1.000x | 1.361x |
| perft_bulk_d4 | 1.043x | 1.023x | 1.454x |
| perft_full_d3 | 1.128x | 1.021x | 1.507x |
| fen_parse | 1.015x | 0.935x | 1.233x |
| fen_write_native_ep | 1.000x | 0.998x | 1.576x |
| snapshot | 1.011x | 1.010x | 0.938x |
| san_parse_all | 0.998x | 0.993x | 1.816x |
| san_render_native | 0.993x | 0.967x | 2.667x |
| apply_children_full | 1.007x | 0.980x | 1.166x |
| make_unmake_full | 1.002x | 0.970x | 1.563x |
| startpos | 0.999x | 0.999x | 1.273x |
| chess960_materialize | 0.979x | 0.944x | 1.455x |
| chess960_perft_d3 | 1.000x | 0.967x | 1.498x |
| polyglot_key | 1.002x | 0.996x | 25.114x |
| polyglot_recompute | 1.364x | 1.275x | 1.474x |
| repetition_key | 0.992x | 0.984x | 18.929x |
| corpus_count | 1.095x | 1.067x | 1.504x |
| corpus_materialize | 1.003x | 1.002x | 1.244x |
| corpus_uci_sorted | 1.008x | 1.005x | 1.459x |
| replay_trusted_native | 0.909x | 0.901x | 0.585x |
| replay_checked_native | 0.952x | 0.946x | 1.073x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.06x).

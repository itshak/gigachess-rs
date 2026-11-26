| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.980x | 1.149x |
| materialize | 0.962x | 0.944x | 1.293x |
| perft_bulk_d4 | 0.994x | 0.982x | 1.146x |
| perft_full_d3 | 0.993x | 0.968x | 1.318x |
| fen_parse | 1.011x | 0.988x | 1.285x |
| fen_write_native_ep | 0.936x | 0.901x | 1.105x |
| snapshot | 1.002x | 0.987x | 1.003x |
| san_parse_all | 0.995x | 0.981x | 1.789x |
| san_render_native | 1.015x | 1.001x | 2.421x |
| apply_children_full | 0.996x | 0.967x | 1.046x |
| make_unmake_full | 0.999x | 0.964x | 1.397x |
| startpos | 0.929x | 0.929x | 1.214x |
| chess960_materialize | 0.995x | 0.988x | 1.366x |
| chess960_perft_d3 | 0.989x | 0.977x | 1.332x |
| polyglot_key | 1.014x | 0.982x | 32.668x |
| polyglot_recompute | 0.987x | 0.980x | 1.012x |
| repetition_key | 1.051x | 0.987x | 25.101x |
| corpus_count | 1.006x | 0.992x | 1.112x |
| corpus_materialize | 1.007x | 0.992x | 1.246x |
| corpus_uci_sorted | 1.005x | 0.986x | 1.603x |
| replay_trusted_native | 0.984x | 0.984x | 0.560x |
| replay_checked_native | 1.022x | 1.019x | 0.995x |

Cold first legal move list: 0.013 ms -> 0.013 ms (1.03x).

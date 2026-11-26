| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.113x | 1.003x | 1.345x |
| materialize | 1.017x | 0.905x | 1.170x |
| perft_bulk_d4 | 0.984x | 0.925x | 1.199x |
| perft_full_d3 | 0.969x | 0.900x | 1.249x |
| fen_parse | 1.190x | 1.162x | 1.319x |
| fen_write_native_ep | 0.994x | 0.989x | 1.611x |
| snapshot | 0.998x | 0.969x | 0.996x |
| san_parse_all | 1.019x | 1.005x | 2.082x |
| san_render_native | 1.011x | 0.997x | 3.876x |
| apply_children_full | 0.997x | 0.979x | 1.357x |
| make_unmake_full | 1.006x | 0.986x | 1.341x |
| startpos | 1.002x | 1.002x | 1.253x |
| chess960_materialize | 1.001x | 0.989x | 1.276x |
| chess960_perft_d3 | 1.001x | 1.000x | 1.426x |
| polyglot_key | 0.995x | 0.995x | 29.392x |
| polyglot_recompute | 1.022x | 1.022x | 1.016x |
| repetition_key | 1.080x | 0.996x | 22.254x |
| corpus_count | 1.143x | 1.007x | 1.216x |
| corpus_materialize | 1.032x | 1.006x | 1.109x |
| corpus_uci_sorted | 1.046x | 0.994x | 1.419x |
| replay_trusted_native | 1.002x | 0.986x | 0.842x |
| replay_checked_native | 1.065x | 1.061x | 1.080x |

Cold first legal move list: 0.016 ms -> 0.019 ms (0.82x).

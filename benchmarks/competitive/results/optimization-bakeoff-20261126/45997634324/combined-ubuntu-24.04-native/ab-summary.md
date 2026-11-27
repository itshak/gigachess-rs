| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.050x | 1.027x | 1.530x |
| materialize | 1.026x | 1.009x | 1.285x |
| perft_bulk_d4 | 1.044x | 1.023x | 1.459x |
| perft_full_d3 | 1.224x | 1.031x | 1.585x |
| fen_parse | 1.061x | 1.036x | 1.178x |
| fen_write_native_ep | 0.984x | 0.972x | 1.695x |
| snapshot | 1.284x | 1.283x | 1.250x |
| san_parse_all | 0.994x | 0.976x | 2.050x |
| san_render_native | 1.005x | 0.979x | 2.209x |
| apply_children_full | 1.001x | 0.986x | 1.328x |
| make_unmake_full | 1.012x | 0.999x | 1.323x |
| startpos | 0.998x | 0.998x | 1.247x |
| chess960_materialize | 1.022x | 1.018x | 1.434x |
| chess960_perft_d3 | 0.992x | 0.986x | 1.522x |
| polyglot_key | 0.998x | 0.998x | 25.852x |
| polyglot_recompute | 1.000x | 0.997x | 1.030x |
| repetition_key | 1.013x | 1.005x | 19.627x |
| corpus_count | 1.075x | 1.059x | 1.486x |
| corpus_materialize | 1.019x | 1.017x | 1.171x |
| corpus_uci_sorted | 1.012x | 1.009x | 1.854x |
| replay_trusted_native | 0.997x | 0.983x | 0.902x |
| replay_checked_native | 0.969x | 0.960x | 1.092x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.19x).

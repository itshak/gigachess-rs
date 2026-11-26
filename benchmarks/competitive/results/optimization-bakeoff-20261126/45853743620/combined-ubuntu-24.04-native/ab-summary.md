| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.998x | 1.506x |
| materialize | 0.999x | 0.993x | 1.580x |
| perft_bulk_d4 | 0.999x | 0.993x | 1.172x |
| perft_full_d3 | 0.998x | 0.994x | 1.371x |
| fen_parse | 0.925x | 0.814x | 1.061x |
| fen_write_native_ep | 0.998x | 0.986x | 1.900x |
| snapshot | 0.999x | 0.997x | 1.154x |
| san_parse_all | 1.124x | 1.091x | 1.944x |
| san_render_native | 1.002x | 0.997x | 2.127x |
| apply_children_full | 0.993x | 0.967x | 1.295x |
| make_unmake_full | 0.910x | 0.864x | 1.072x |
| startpos | 0.996x | 0.996x | 1.132x |
| chess960_materialize | 1.092x | 1.081x | 1.536x |
| chess960_perft_d3 | 0.999x | 0.996x | 1.516x |
| polyglot_key | 1.004x | 0.997x | 38.916x |
| polyglot_recompute | 0.943x | 0.928x | 0.958x |
| repetition_key | 0.833x | 0.788x | 19.300x |
| corpus_count | 1.003x | 0.996x | 1.114x |
| corpus_materialize | 1.015x | 1.013x | 1.075x |
| corpus_uci_sorted | 1.005x | 1.001x | 1.557x |
| replay_trusted_native | 1.049x | 0.964x | 0.907x |
| replay_checked_native | 1.011x | 1.003x | 1.121x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.96x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 1.029x | 1.354x |
| materialize | 1.093x | 1.058x | 1.156x |
| perft_bulk_d4 | 1.181x | 1.142x | 1.228x |
| perft_full_d3 | 1.005x | 0.954x | 0.624x |
| fen_parse | 1.119x | 1.114x | 1.124x |
| fen_write_native_ep | 1.013x | 0.989x | 1.233x |
| snapshot | 1.080x | 1.078x | 0.832x |
| san_parse_all | 1.419x | 1.357x | 1.403x |
| san_render_native | 1.477x | 1.401x | 1.675x |
| apply_children_full | 1.125x | 1.090x | 0.752x |
| make_unmake_full | 1.067x | 1.047x | 0.904x |
| startpos | 1.061x | 1.061x | 0.750x |
| chess960_materialize | 1.209x | 1.147x | 1.427x |
| chess960_perft_d3 | 1.296x | 1.166x | 1.284x |
| polyglot_key | 0.967x | 0.941x | 22.366x |
| polyglot_recompute | 1.005x | 1.004x | 1.055x |
| repetition_key | 1.058x | 1.029x | 13.635x |
| corpus_count | 1.049x | 1.047x | 1.386x |
| corpus_materialize | 1.019x | 1.016x | 1.248x |
| corpus_uci_sorted | 1.202x | 1.200x | 0.767x |
| replay_trusted_native | 1.106x | 1.043x | 0.575x |
| replay_checked_native | 1.200x | 1.106x | 0.852x |

Cold first legal move list: 1.032 ms → 0.012 ms (87.98x).

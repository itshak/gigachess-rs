| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.966x | 1.300x |
| materialize | 0.963x | 0.865x | 1.097x |
| perft_bulk_d4 | 0.988x | 0.963x | 1.223x |
| perft_full_d3 | 1.008x | 0.925x | 0.884x |
| fen_parse | 0.955x | 0.886x | 1.337x |
| fen_write_native_ep | 1.009x | 0.993x | 1.254x |
| snapshot | 0.897x | 0.891x | 1.003x |
| san_parse_all | 1.042x | 1.007x | 2.022x |
| san_render_native | 1.013x | 0.994x | 3.528x |
| apply_children_full | 0.904x | 0.839x | 0.721x |
| make_unmake_full | 0.971x | 0.912x | 1.229x |
| startpos | 0.997x | 0.997x | 1.242x |
| chess960_materialize | 1.010x | 1.005x | 1.182x |
| chess960_perft_d3 | 1.001x | 0.996x | 1.362x |
| polyglot_key | 1.040x | 1.014x | 38.139x |
| polyglot_recompute | 1.015x | 1.005x | 1.012x |
| repetition_key | 1.003x | 1.002x | 25.453x |
| corpus_count | 0.996x | 0.992x | 1.292x |
| corpus_materialize | 1.024x | 1.002x | 1.163x |
| corpus_uci_sorted | 1.006x | 1.002x | 1.398x |
| replay_trusted_native | 0.998x | 0.998x | 0.715x |
| replay_checked_native | 0.971x | 0.971x | 0.926x |

Cold first legal move list: 0.020 ms -> 0.020 ms (1.00x).

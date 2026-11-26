| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.781x | 0.577x | 1.414x |
| materialize | 0.997x | 0.762x | 1.127x |
| perft_bulk_d4 | 0.965x | 0.864x | 1.067x |
| perft_full_d3 | 0.723x | 0.634x | 0.989x |
| fen_parse | 0.980x | 0.780x | 1.131x |
| fen_write_native_ep | 0.841x | 0.757x | 1.409x |
| snapshot | 1.253x | 0.856x | 1.082x |
| san_parse_all | 1.007x | 0.838x | 1.877x |
| san_render_native | 0.909x | 0.736x | 2.665x |
| apply_children_full | 0.888x | 0.608x | 1.017x |
| make_unmake_full | 0.725x | 0.614x | 1.014x |
| startpos | 2.318x | 2.318x | 2.981x |
| chess960_materialize | 0.919x | 0.888x | 1.487x |
| chess960_perft_d3 | 0.919x | 0.832x | 1.285x |
| polyglot_key | 1.055x | 1.002x | 45.688x |
| polyglot_recompute | 1.080x | 1.019x | 0.893x |
| repetition_key | 1.103x | 1.094x | 19.632x |
| corpus_count | 0.951x | 0.831x | 1.301x |
| corpus_materialize | 0.993x | 0.965x | 1.234x |
| corpus_uci_sorted | 0.962x | 0.898x | 1.348x |
| replay_trusted_native | 0.944x | 0.930x | 0.642x |
| replay_checked_native | 0.852x | 0.773x | 1.013x |

Cold first legal move list: 0.003 ms -> 0.007 ms (0.45x).

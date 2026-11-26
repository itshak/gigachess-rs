| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.996x | 1.560x |
| materialize | 1.003x | 0.995x | 1.318x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.397x |
| perft_full_d3 | 0.998x | 0.993x | 1.333x |
| fen_parse | 1.003x | 0.997x | 1.223x |
| fen_write_native_ep | 1.001x | 0.999x | 1.624x |
| snapshot | 1.017x | 1.010x | 0.938x |
| san_parse_all | 0.999x | 0.993x | 1.834x |
| san_render_native | 1.006x | 0.998x | 2.672x |
| apply_children_full | 1.004x | 0.990x | 1.160x |
| make_unmake_full | 0.999x | 0.985x | 1.564x |
| startpos | 1.001x | 1.001x | 1.279x |
| chess960_materialize | 1.003x | 1.002x | 1.500x |
| chess960_perft_d3 | 1.005x | 1.001x | 1.507x |
| polyglot_key | 1.013x | 1.008x | 26.231x |
| polyglot_recompute | 0.997x | 0.996x | 1.088x |
| repetition_key | 1.002x | 1.001x | 19.609x |
| corpus_count | 1.006x | 1.005x | 1.359x |
| corpus_materialize | 0.999x | 0.998x | 1.218x |
| corpus_uci_sorted | 0.992x | 0.992x | 1.456x |
| replay_trusted_native | 0.986x | 0.973x | 0.628x |
| replay_checked_native | 1.004x | 1.003x | 1.132x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).

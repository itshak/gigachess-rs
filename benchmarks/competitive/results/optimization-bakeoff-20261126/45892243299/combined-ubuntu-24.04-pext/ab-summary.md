| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.988x | 1.557x |
| materialize | 0.991x | 0.951x | 1.616x |
| perft_bulk_d4 | 1.001x | 0.992x | 1.193x |
| perft_full_d3 | 1.004x | 0.992x | 1.362x |
| fen_parse | 0.979x | 0.934x | 1.117x |
| fen_write_native_ep | 1.012x | 0.970x | 1.957x |
| snapshot | 0.995x | 0.986x | 1.326x |
| san_parse_all | 0.906x | 0.886x | 1.672x |
| san_render_native | 1.028x | 1.020x | 2.176x |
| apply_children_full | 0.995x | 0.988x | 1.256x |
| make_unmake_full | 0.984x | 0.972x | 1.180x |
| startpos | 1.242x | 1.242x | 1.239x |
| chess960_materialize | 0.995x | 0.981x | 1.506x |
| chess960_perft_d3 | 0.993x | 0.992x | 1.471x |
| polyglot_key | 1.003x | 0.999x | 39.286x |
| polyglot_recompute | 1.101x | 0.902x | 1.111x |
| repetition_key | 0.961x | 0.910x | 22.565x |
| corpus_count | 0.998x | 0.997x | 1.134x |
| corpus_materialize | 1.013x | 1.013x | 1.098x |
| corpus_uci_sorted | 1.020x | 1.017x | 1.581x |
| replay_trusted_native | 0.978x | 0.915x | 0.867x |
| replay_checked_native | 1.002x | 0.998x | 1.122x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.965x | 0.938x | 1.235x |
| materialize | 0.988x | 0.978x | 1.742x |
| perft_bulk_d4 | 1.006x | 0.965x | 1.201x |
| perft_full_d3 | 0.988x | 0.973x | 1.306x |
| fen_parse | 1.011x | 1.003x | 1.059x |
| fen_write_native_ep | 1.004x | 0.991x | 1.873x |
| snapshot | 1.002x | 0.994x | 1.000x |
| san_parse_all | 1.003x | 0.998x | 1.895x |
| san_render_native | 0.974x | 0.954x | 2.106x |
| apply_children_full | 1.000x | 0.996x | 1.347x |
| make_unmake_full | 1.045x | 0.987x | 1.249x |
| startpos | 0.994x | 0.994x | 1.436x |
| chess960_materialize | 1.010x | 1.004x | 1.576x |
| chess960_perft_d3 | 0.975x | 0.967x | 1.468x |
| polyglot_key | 1.078x | 1.000x | 42.730x |
| polyglot_recompute | 1.069x | 1.047x | 1.075x |
| repetition_key | 1.014x | 1.000x | 21.554x |
| corpus_count | 1.109x | 1.057x | 1.265x |
| corpus_materialize | 0.988x | 0.987x | 1.127x |
| corpus_uci_sorted | 1.000x | 0.997x | 1.564x |
| replay_trusted_native | 0.999x | 0.997x | 0.863x |
| replay_checked_native | 0.999x | 0.994x | 1.160x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).

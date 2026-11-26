| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.998x | 1.550x |
| materialize | 0.999x | 0.995x | 1.326x |
| perft_bulk_d4 | 1.003x | 0.997x | 1.392x |
| perft_full_d3 | 0.999x | 0.977x | 0.833x |
| fen_parse | 1.003x | 0.992x | 1.210x |
| fen_write_native_ep | 0.999x | 0.996x | 1.571x |
| snapshot | 1.003x | 0.996x | 0.933x |
| san_parse_all | 1.002x | 0.998x | 1.807x |
| san_render_native | 1.009x | 0.994x | 2.549x |
| apply_children_full | 1.002x | 0.984x | 1.163x |
| make_unmake_full | 0.999x | 0.976x | 1.542x |
| startpos | 1.000x | 1.000x | 1.274x |
| chess960_materialize | 1.002x | 0.999x | 1.490x |
| chess960_perft_d3 | 1.005x | 1.004x | 1.509x |
| polyglot_key | 0.994x | 0.990x | 25.975x |
| polyglot_recompute | 1.042x | 1.030x | 1.138x |
| repetition_key | 0.997x | 0.996x | 19.636x |
| corpus_count | 1.017x | 1.002x | 1.343x |
| corpus_materialize | 1.004x | 0.996x | 1.213x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.460x |
| replay_trusted_native | 0.996x | 0.994x | 0.614x |
| replay_checked_native | 0.996x | 0.992x | 1.038x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).

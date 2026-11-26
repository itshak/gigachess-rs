| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.083x |
| materialize | 0.999x | 0.996x | 1.178x |
| perft_bulk_d4 | 1.017x | 0.992x | 0.919x |
| perft_full_d3 | 1.072x | 1.047x | 0.656x |
| fen_parse | 1.029x | 1.020x | 1.239x |
| fen_write_native_ep | 0.999x | 0.992x | 1.069x |
| snapshot | 0.992x | 0.982x | 0.781x |
| san_parse_all | 1.090x | 1.059x | 0.995x |
| san_render_native | 1.068x | 1.041x | 1.221x |
| apply_children_full | 1.070x | 1.039x | 0.702x |
| make_unmake_full | 1.082x | 1.048x | 0.965x |
| startpos | 0.937x | 0.937x | 0.813x |
| chess960_materialize | 1.072x | 0.994x | 1.190x |
| chess960_perft_d3 | 1.081x | 1.041x | 0.914x |
| polyglot_key | 0.999x | 0.997x | 24.326x |
| polyglot_recompute | 0.999x | 0.998x | 1.130x |
| repetition_key | 1.026x | 1.020x | 14.978x |
| corpus_count | 1.007x | 1.005x | 1.192x |
| corpus_materialize | 1.004x | 1.004x | 1.137x |
| corpus_uci_sorted | 0.999x | 0.998x | 0.575x |
| replay_trusted_native | 1.064x | 1.043x | 0.342x |
| replay_checked_native | 1.116x | 1.094x | 0.737x |

Cold first legal move list: 154.021 ms → 154.026 ms (1.00x).

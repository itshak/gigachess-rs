| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.717x | 1.270x |
| materialize | 0.973x | 0.908x | 1.191x |
| perft_bulk_d4 | 0.997x | 0.978x | 1.244x |
| perft_full_d3 | 1.009x | 0.930x | 0.964x |
| fen_parse | 1.070x | 0.899x | 1.081x |
| fen_write_native_ep | 1.003x | 0.996x | 1.120x |
| snapshot | 1.157x | 1.138x | 1.173x |
| san_parse_all | 1.006x | 0.997x | 1.836x |
| san_render_native | 1.001x | 0.997x | 2.233x |
| apply_children_full | 1.098x | 1.021x | 1.137x |
| make_unmake_full | 0.972x | 0.901x | 1.225x |
| startpos | 0.999x | 0.999x | 1.254x |
| chess960_materialize | 1.017x | 1.010x | 1.415x |
| chess960_perft_d3 | 1.022x | 1.022x | 1.403x |
| polyglot_key | 1.049x | 1.036x | 36.471x |
| polyglot_recompute | 1.056x | 1.003x | 0.985x |
| repetition_key | 1.022x | 1.002x | 28.415x |
| corpus_count | 1.001x | 1.001x | 1.302x |
| corpus_materialize | 0.987x | 0.955x | 1.223x |
| corpus_uci_sorted | 1.026x | 1.017x | 1.710x |
| replay_trusted_native | 1.106x | 1.012x | 0.730x |
| replay_checked_native | 1.216x | 1.132x | 1.091x |

Cold first legal move list: 0.012 ms -> 0.014 ms (0.88x).

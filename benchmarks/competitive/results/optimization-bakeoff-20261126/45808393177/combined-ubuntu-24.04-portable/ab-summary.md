| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.967x | 1.223x |
| materialize | 0.992x | 0.943x | 1.696x |
| perft_bulk_d4 | 0.908x | 0.860x | 1.059x |
| perft_full_d3 | 0.950x | 0.877x | 1.450x |
| fen_parse | 1.011x | 0.998x | 1.185x |
| fen_write_native_ep | 1.006x | 0.996x | 1.835x |
| snapshot | 1.244x | 1.228x | 1.004x |
| san_parse_all | 0.987x | 0.955x | 1.872x |
| san_render_native | 0.995x | 0.983x | 2.193x |
| apply_children_full | 1.022x | 0.999x | 1.281x |
| make_unmake_full | 1.012x | 0.908x | 1.194x |
| startpos | 0.997x | 0.997x | 1.295x |
| chess960_materialize | 1.048x | 0.981x | 1.416x |
| chess960_perft_d3 | 1.048x | 0.986x | 1.397x |
| polyglot_key | 1.097x | 0.999x | 40.189x |
| polyglot_recompute | 1.008x | 1.007x | 0.990x |
| repetition_key | 0.787x | 0.743x | 16.702x |
| corpus_count | 0.818x | 0.802x | 1.036x |
| corpus_materialize | 0.934x | 0.915x | 1.092x |
| corpus_uci_sorted | 0.982x | 0.979x | 1.506x |
| replay_trusted_native | 0.971x | 0.939x | 0.809x |
| replay_checked_native | 1.003x | 0.995x | 1.098x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.57x).

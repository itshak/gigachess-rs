| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.994x | 1.482x |
| materialize | 0.988x | 0.979x | 1.535x |
| perft_bulk_d4 | 0.998x | 0.978x | 1.261x |
| perft_full_d3 | 0.997x | 0.996x | 1.533x |
| fen_parse | 1.006x | 0.988x | 0.996x |
| fen_write_native_ep | 0.996x | 0.989x | 1.211x |
| snapshot | 2.002x | 1.989x | 0.997x |
| san_parse_all | 0.995x | 0.989x | 1.881x |
| san_render_native | 1.027x | 1.022x | 2.169x |
| apply_children_full | 1.019x | 1.000x | 1.153x |
| make_unmake_full | 1.001x | 0.979x | 1.143x |
| startpos | 1.003x | 1.003x | 1.339x |
| chess960_materialize | 0.995x | 0.993x | 1.460x |
| chess960_perft_d3 | 0.999x | 0.996x | 1.393x |
| polyglot_key | 0.998x | 0.997x | 39.900x |
| polyglot_recompute | 0.997x | 0.995x | 1.021x |
| repetition_key | 0.857x | 0.748x | 19.446x |
| corpus_count | 0.999x | 0.994x | 1.398x |
| corpus_materialize | 0.999x | 0.998x | 1.116x |
| corpus_uci_sorted | 0.996x | 0.995x | 1.564x |
| replay_trusted_native | 0.998x | 0.986x | 0.722x |
| replay_checked_native | 0.995x | 0.984x | 1.109x |

Cold first legal move list: 0.008 ms -> 0.010 ms (0.87x).

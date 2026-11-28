| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.983x | 1.565x |
| materialize | 1.023x | 1.010x | 1.344x |
| perft_bulk_d4 | 1.041x | 1.018x | 1.448x |
| perft_full_d3 | 1.131x | 0.998x | 1.503x |
| fen_parse | 1.018x | 0.942x | 1.237x |
| fen_write_native_ep | 0.994x | 0.989x | 1.577x |
| snapshot | 0.980x | 0.977x | 0.955x |
| san_parse_all | 0.995x | 0.991x | 1.822x |
| san_render_native | 0.999x | 0.991x | 2.663x |
| apply_children_full | 0.998x | 0.971x | 1.151x |
| make_unmake_full | 1.021x | 1.012x | 1.594x |
| startpos | 0.999x | 0.999x | 1.328x |
| chess960_materialize | 1.013x | 1.012x | 1.506x |
| chess960_perft_d3 | 1.015x | 1.002x | 1.519x |
| polyglot_key | 1.027x | 1.009x | 25.996x |
| polyglot_recompute | 1.368x | 1.282x | 1.484x |
| repetition_key | 1.010x | 0.999x | 19.512x |
| corpus_count | 1.076x | 1.043x | 1.444x |
| corpus_materialize | 1.002x | 1.002x | 1.223x |
| corpus_uci_sorted | 1.000x | 0.997x | 1.464x |
| replay_trusted_native | 1.009x | 1.006x | 0.633x |
| replay_checked_native | 1.015x | 1.012x | 1.146x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.84x).

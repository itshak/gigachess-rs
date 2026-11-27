| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.280x |
| materialize | 0.998x | 0.993x | 1.790x |
| perft_bulk_d4 | 0.989x | 0.985x | 1.174x |
| perft_full_d3 | 1.006x | 0.998x | 1.345x |
| fen_parse | 1.024x | 1.004x | 1.178x |
| fen_write_native_ep | 1.014x | 0.977x | 1.904x |
| snapshot | 1.000x | 0.999x | 1.002x |
| san_parse_all | 1.012x | 1.002x | 1.922x |
| san_render_native | 1.014x | 1.001x | 2.185x |
| apply_children_full | 0.993x | 0.977x | 1.372x |
| make_unmake_full | 1.031x | 0.943x | 1.237x |
| startpos | 1.000x | 1.000x | 1.286x |
| chess960_materialize | 1.005x | 0.999x | 1.561x |
| chess960_perft_d3 | 1.000x | 0.995x | 1.504x |
| polyglot_key | 0.992x | 0.985x | 40.608x |
| polyglot_recompute | 1.026x | 1.013x | 0.989x |
| repetition_key | 0.940x | 0.893x | 19.441x |
| corpus_count | 1.038x | 1.001x | 1.151x |
| corpus_materialize | 0.996x | 0.994x | 1.149x |
| corpus_uci_sorted | 1.004x | 1.002x | 1.556x |
| replay_trusted_native | 1.061x | 1.053x | 0.935x |
| replay_checked_native | 1.010x | 1.001x | 1.168x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.99x).

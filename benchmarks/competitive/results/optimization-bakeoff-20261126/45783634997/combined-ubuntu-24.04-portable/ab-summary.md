| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 1.000x | 1.178x |
| materialize | 0.985x | 0.968x | 1.674x |
| perft_bulk_d4 | 1.002x | 0.996x | 1.161x |
| perft_full_d3 | 1.010x | 1.000x | 1.493x |
| fen_parse | 0.947x | 0.921x | 1.131x |
| fen_write_native_ep | 1.553x | 1.472x | 1.768x |
| snapshot | 0.996x | 0.989x | 0.994x |
| san_parse_all | 1.003x | 0.973x | 1.858x |
| san_render_native | 1.008x | 0.989x | 2.183x |
| apply_children_full | 1.000x | 0.982x | 1.099x |
| make_unmake_full | 1.010x | 0.991x | 1.244x |
| startpos | 0.994x | 0.994x | 1.292x |
| chess960_materialize | 0.985x | 0.984x | 1.407x |
| chess960_perft_d3 | 0.996x | 0.994x | 1.343x |
| polyglot_key | 1.074x | 0.996x | 42.824x |
| polyglot_recompute | 0.968x | 0.964x | 1.038x |
| repetition_key | 0.919x | 0.912x | 22.878x |
| corpus_count | 1.003x | 0.997x | 1.264x |
| corpus_materialize | 0.988x | 0.984x | 1.136x |
| corpus_uci_sorted | 1.001x | 0.994x | 1.557x |
| replay_trusted_native | 0.991x | 0.982x | 0.693x |
| replay_checked_native | 0.991x | 0.988x | 1.113x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.90x).

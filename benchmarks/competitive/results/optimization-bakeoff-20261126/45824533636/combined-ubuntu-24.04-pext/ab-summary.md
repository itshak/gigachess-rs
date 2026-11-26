| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.972x | 1.294x |
| materialize | 1.014x | 0.983x | 1.244x |
| perft_bulk_d4 | 0.998x | 0.981x | 1.222x |
| perft_full_d3 | 1.018x | 1.001x | 1.229x |
| fen_parse | 1.136x | 1.085x | 1.193x |
| fen_write_native_ep | 1.007x | 0.990x | 1.737x |
| snapshot | 1.003x | 1.000x | 1.249x |
| san_parse_all | 1.018x | 0.999x | 2.110x |
| san_render_native | 0.975x | 0.955x | 2.177x |
| apply_children_full | 1.016x | 0.997x | 1.322x |
| make_unmake_full | 0.992x | 0.979x | 1.267x |
| startpos | 0.999x | 0.999x | 1.252x |
| chess960_materialize | 0.994x | 0.983x | 1.314x |
| chess960_perft_d3 | 0.975x | 0.974x | 1.349x |
| polyglot_key | 0.999x | 0.995x | 29.889x |
| polyglot_recompute | 0.976x | 0.973x | 1.018x |
| repetition_key | 1.000x | 0.990x | 19.262x |
| corpus_count | 1.006x | 0.999x | 1.295x |
| corpus_materialize | 1.014x | 1.013x | 1.136x |
| corpus_uci_sorted | 1.006x | 0.989x | 1.719x |
| replay_trusted_native | 0.999x | 0.991x | 0.852x |
| replay_checked_native | 0.999x | 0.999x | 1.025x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.93x).

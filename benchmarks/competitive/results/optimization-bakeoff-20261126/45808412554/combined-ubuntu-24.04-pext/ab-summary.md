| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.099x | 0.989x | 1.627x |
| materialize | 1.100x | 1.058x | 1.621x |
| perft_bulk_d4 | 1.065x | 1.008x | 1.336x |
| perft_full_d3 | 1.058x | 1.033x | 1.543x |
| fen_parse | 0.959x | 0.938x | 1.127x |
| fen_write_native_ep | 1.006x | 0.969x | 2.042x |
| snapshot | 0.498x | 0.491x | 0.998x |
| san_parse_all | 0.871x | 0.843x | 1.689x |
| san_render_native | 1.014x | 0.999x | 2.171x |
| apply_children_full | 0.982x | 0.948x | 1.544x |
| make_unmake_full | 1.000x | 0.992x | 1.112x |
| startpos | 0.830x | 0.830x | 1.331x |
| chess960_materialize | 1.060x | 1.039x | 1.527x |
| chess960_perft_d3 | 1.061x | 1.046x | 1.461x |
| polyglot_key | 1.102x | 1.009x | 39.578x |
| polyglot_recompute | 0.970x | 0.957x | 0.980x |
| repetition_key | 1.030x | 0.985x | 19.311x |
| corpus_count | 1.039x | 0.953x | 1.448x |
| corpus_materialize | 0.999x | 0.994x | 1.104x |
| corpus_uci_sorted | 0.995x | 0.990x | 1.553x |
| replay_trusted_native | 1.004x | 1.001x | 0.821x |
| replay_checked_native | 1.000x | 0.997x | 1.100x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.92x).

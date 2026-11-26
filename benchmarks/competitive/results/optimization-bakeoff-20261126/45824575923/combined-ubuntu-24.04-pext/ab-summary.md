| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.105x | 0.994x | 1.476x |
| materialize | 1.046x | 0.989x | 1.234x |
| perft_bulk_d4 | 1.093x | 1.009x | 1.307x |
| perft_full_d3 | 0.993x | 0.918x | 1.192x |
| fen_parse | 0.990x | 0.944x | 1.035x |
| fen_write_native_ep | 0.993x | 0.980x | 1.447x |
| snapshot | 0.997x | 0.972x | 1.484x |
| san_parse_all | 0.999x | 0.981x | 1.775x |
| san_render_native | 1.016x | 0.962x | 2.269x |
| apply_children_full | 1.345x | 1.244x | 1.320x |
| make_unmake_full | 0.979x | 0.920x | 1.208x |
| startpos | 1.005x | 1.005x | 1.251x |
| chess960_materialize | 1.058x | 1.051x | 1.498x |
| chess960_perft_d3 | 1.097x | 1.075x | 1.531x |
| polyglot_key | 0.989x | 0.982x | 35.016x |
| polyglot_recompute | 1.004x | 0.992x | 0.969x |
| repetition_key | 1.027x | 0.980x | 21.988x |
| corpus_count | 1.113x | 1.054x | 1.193x |
| corpus_materialize | 1.086x | 1.056x | 1.140x |
| corpus_uci_sorted | 1.063x | 1.046x | 1.721x |
| replay_trusted_native | 0.965x | 0.957x | 0.675x |
| replay_checked_native | 1.018x | 0.987x | 1.131x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.95x).

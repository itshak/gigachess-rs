| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.964x | 1.223x |
| materialize | 0.977x | 0.920x | 1.642x |
| perft_bulk_d4 | 0.919x | 0.867x | 1.079x |
| perft_full_d3 | 0.972x | 0.922x | 1.471x |
| fen_parse | 0.997x | 0.979x | 1.183x |
| fen_write_native_ep | 0.982x | 0.963x | 1.789x |
| snapshot | 1.250x | 1.243x | 1.004x |
| san_parse_all | 1.008x | 0.981x | 2.041x |
| san_render_native | 0.994x | 0.971x | 2.202x |
| apply_children_full | 1.003x | 0.982x | 1.097x |
| make_unmake_full | 1.004x | 0.984x | 1.199x |
| startpos | 0.989x | 0.989x | 1.408x |
| chess960_materialize | 1.028x | 0.968x | 1.464x |
| chess960_perft_d3 | 1.037x | 0.977x | 1.388x |
| polyglot_key | 1.004x | 1.002x | 40.834x |
| polyglot_recompute | 0.981x | 0.965x | 0.993x |
| repetition_key | 1.034x | 0.858x | 21.450x |
| corpus_count | 0.825x | 0.814x | 1.044x |
| corpus_materialize | 0.958x | 0.946x | 1.082x |
| corpus_uci_sorted | 0.990x | 0.988x | 1.549x |
| replay_trusted_native | 1.024x | 1.020x | 0.727x |
| replay_checked_native | 0.993x | 0.983x | 1.060x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.52x).

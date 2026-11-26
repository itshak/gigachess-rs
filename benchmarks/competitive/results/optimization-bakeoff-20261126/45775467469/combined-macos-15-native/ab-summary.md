| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.109x | 1.041x | 1.231x |
| materialize | 1.095x | 1.055x | 1.004x |
| perft_bulk_d4 | 1.125x | 1.032x | 1.261x |
| perft_full_d3 | 1.807x | 1.537x | 1.236x |
| fen_parse | 0.979x | 0.863x | 1.174x |
| fen_write_native_ep | 1.057x | 0.922x | 1.251x |
| snapshot | 1.170x | 0.964x | 1.010x |
| san_parse_all | 1.106x | 0.971x | 1.587x |
| san_render_native | 1.182x | 1.004x | 2.675x |
| apply_children_full | 1.250x | 1.003x | 0.810x |
| make_unmake_full | 1.228x | 1.051x | 1.013x |
| startpos | 1.026x | 1.026x | 1.229x |
| chess960_materialize | 0.997x | 0.933x | 1.213x |
| chess960_perft_d3 | 1.061x | 1.052x | 1.397x |
| polyglot_key | 1.040x | 0.992x | 40.190x |
| polyglot_recompute | 0.995x | 0.883x | 0.944x |
| repetition_key | 1.158x | 1.134x | 19.512x |
| corpus_count | 0.963x | 0.914x | 1.497x |
| corpus_materialize | 0.931x | 0.828x | 1.001x |
| corpus_uci_sorted | 0.993x | 0.941x | 0.932x |
| replay_trusted_native | 0.962x | 0.936x | 0.443x |
| replay_checked_native | 0.954x | 0.900x | 0.835x |

Cold first legal move list: 0.006 ms -> 0.023 ms (0.27x).

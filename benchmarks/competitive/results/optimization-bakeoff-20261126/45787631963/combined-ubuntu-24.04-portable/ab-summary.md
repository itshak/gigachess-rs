| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.932x | 1.194x |
| materialize | 0.962x | 0.917x | 1.334x |
| perft_bulk_d4 | 0.967x | 0.950x | 1.170x |
| perft_full_d3 | 0.954x | 0.927x | 1.308x |
| fen_parse | 1.054x | 1.017x | 1.335x |
| fen_write_native_ep | 0.959x | 0.922x | 1.611x |
| snapshot | 1.000x | 0.997x | 1.009x |
| san_parse_all | 1.031x | 1.013x | 2.057x |
| san_render_native | 0.998x | 0.975x | 2.408x |
| apply_children_full | 1.007x | 0.995x | 1.157x |
| make_unmake_full | 1.019x | 1.016x | 1.336x |
| startpos | 0.993x | 0.993x | 1.280x |
| chess960_materialize | 1.023x | 0.972x | 1.400x |
| chess960_perft_d3 | 1.041x | 0.985x | 1.444x |
| polyglot_key | 0.999x | 0.997x | 23.251x |
| polyglot_recompute | 0.988x | 0.983x | 1.019x |
| repetition_key | 1.042x | 1.003x | 18.791x |
| corpus_count | 0.910x | 0.850x | 1.196x |
| corpus_materialize | 0.938x | 0.912x | 1.154x |
| corpus_uci_sorted | 0.972x | 0.968x | 1.795x |
| replay_trusted_native | 1.005x | 1.004x | 0.691x |
| replay_checked_native | 1.007x | 1.003x | 1.063x |

Cold first legal move list: 0.014 ms -> 0.008 ms (1.80x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.812x | 1.301x |
| materialize | 0.960x | 0.867x | 0.929x |
| perft_bulk_d4 | 1.137x | 0.994x | 1.476x |
| perft_full_d3 | 1.036x | 0.770x | 1.374x |
| fen_parse | 0.884x | 0.742x | 1.153x |
| fen_write_native_ep | 0.876x | 0.759x | 1.672x |
| snapshot | 0.959x | 0.681x | 0.947x |
| san_parse_all | 0.800x | 0.669x | 1.819x |
| san_render_native | 0.864x | 0.756x | 2.609x |
| apply_children_full | 0.910x | 0.800x | 0.935x |
| make_unmake_full | 0.917x | 0.743x | 1.218x |
| startpos | 1.058x | 1.058x | 1.164x |
| chess960_materialize | 0.962x | 0.891x | 1.549x |
| chess960_perft_d3 | 0.830x | 0.777x | 1.411x |
| polyglot_key | 1.149x | 1.109x | 40.681x |
| polyglot_recompute | 1.241x | 1.211x | 1.006x |
| repetition_key | 0.967x | 0.815x | 18.081x |
| corpus_count | 0.916x | 0.763x | 1.047x |
| corpus_materialize | 1.192x | 1.005x | 1.110x |
| corpus_uci_sorted | 1.028x | 0.972x | 1.318x |
| replay_trusted_native | 1.048x | 1.008x | 0.614x |
| replay_checked_native | 1.042x | 0.860x | 0.969x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.97x).

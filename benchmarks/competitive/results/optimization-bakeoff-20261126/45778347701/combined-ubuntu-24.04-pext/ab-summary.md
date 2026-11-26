| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 1.004x | 1.539x |
| materialize | 0.948x | 0.882x | 1.460x |
| perft_bulk_d4 | 0.916x | 0.882x | 1.149x |
| perft_full_d3 | 1.006x | 0.977x | 1.546x |
| fen_parse | 0.961x | 0.905x | 0.956x |
| fen_write_native_ep | 1.005x | 0.978x | 1.349x |
| snapshot | 1.061x | 1.051x | 0.748x |
| san_parse_all | 0.974x | 0.927x | 1.842x |
| san_render_native | 0.988x | 0.941x | 2.114x |
| apply_children_full | 0.693x | 0.668x | 0.806x |
| make_unmake_full | 0.853x | 0.797x | 0.970x |
| startpos | 0.978x | 0.978x | 1.003x |
| chess960_materialize | 1.001x | 0.999x | 1.329x |
| chess960_perft_d3 | 0.920x | 0.882x | 1.281x |
| polyglot_key | 0.989x | 0.978x | 38.917x |
| polyglot_recompute | 1.015x | 1.004x | 1.025x |
| repetition_key | 0.939x | 0.916x | 20.664x |
| corpus_count | 1.013x | 1.009x | 1.417x |
| corpus_materialize | 1.006x | 1.006x | 1.138x |
| corpus_uci_sorted | 1.013x | 1.008x | 1.599x |
| replay_trusted_native | 0.954x | 0.919x | 0.682x |
| replay_checked_native | 0.951x | 0.948x | 1.005x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.11x).

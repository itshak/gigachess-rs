| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.020x | 0.995x | 1.251x |
| materialize | 1.132x | 1.072x | 1.355x |
| perft_bulk_d4 | 0.990x | 0.943x | 1.208x |
| perft_full_d3 | 0.992x | 0.963x | 1.340x |
| fen_parse | 1.073x | 1.045x | 1.282x |
| fen_write_native_ep | 1.017x | 0.978x | 1.469x |
| snapshot | 0.995x | 0.975x | 0.995x |
| san_parse_all | 1.012x | 0.992x | 1.820x |
| san_render_native | 1.013x | 0.957x | 2.403x |
| apply_children_full | 1.007x | 0.963x | 1.269x |
| make_unmake_full | 1.015x | 0.977x | 1.317x |
| startpos | 1.024x | 1.024x | 1.289x |
| chess960_materialize | 1.019x | 1.009x | 1.600x |
| chess960_perft_d3 | 1.006x | 1.005x | 1.494x |
| polyglot_key | 0.976x | 0.948x | 32.068x |
| polyglot_recompute | 1.018x | 1.011x | 1.015x |
| repetition_key | 1.185x | 1.137x | 24.725x |
| corpus_count | 1.011x | 0.980x | 1.103x |
| corpus_materialize | 1.020x | 0.992x | 1.269x |
| corpus_uci_sorted | 0.989x | 0.959x | 1.565x |
| replay_trusted_native | 1.086x | 1.027x | 0.781x |
| replay_checked_native | 0.956x | 0.946x | 1.261x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.87x).

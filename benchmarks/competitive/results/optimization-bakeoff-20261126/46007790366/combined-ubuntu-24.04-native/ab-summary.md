| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.049x | 1.009x | 1.579x |
| materialize | 1.016x | 1.004x | 1.255x |
| perft_bulk_d4 | 1.047x | 1.037x | 1.466x |
| perft_full_d3 | 1.234x | 1.030x | 1.574x |
| fen_parse | 1.139x | 1.080x | 1.165x |
| fen_write_native_ep | 0.997x | 0.986x | 1.744x |
| snapshot | 1.007x | 1.000x | 1.249x |
| san_parse_all | 1.000x | 0.990x | 2.042x |
| san_render_native | 0.984x | 0.965x | 2.156x |
| apply_children_full | 1.008x | 0.990x | 1.334x |
| make_unmake_full | 0.987x | 0.974x | 1.252x |
| startpos | 0.999x | 0.999x | 1.252x |
| chess960_materialize | 1.004x | 0.989x | 1.460x |
| chess960_perft_d3 | 0.976x | 0.952x | 1.487x |
| polyglot_key | 0.992x | 0.975x | 24.891x |
| polyglot_recompute | 0.968x | 0.875x | 0.991x |
| repetition_key | 1.000x | 0.995x | 18.849x |
| corpus_count | 1.075x | 1.055x | 1.468x |
| corpus_materialize | 1.022x | 1.017x | 1.173x |
| corpus_uci_sorted | 1.006x | 0.999x | 1.738x |
| replay_trusted_native | 0.977x | 0.971x | 0.873x |
| replay_checked_native | 0.999x | 0.999x | 1.068x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.00x).

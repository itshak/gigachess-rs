| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.120x | 0.948x | 1.484x |
| materialize | 1.018x | 0.889x | 1.185x |
| perft_bulk_d4 | 1.075x | 0.965x | 1.366x |
| perft_full_d3 | 1.042x | 0.998x | 1.292x |
| fen_parse | 0.979x | 0.914x | 1.285x |
| fen_write_native_ep | 0.994x | 0.964x | 1.507x |
| snapshot | 0.999x | 0.967x | 1.495x |
| san_parse_all | 0.998x | 0.967x | 1.953x |
| san_render_native | 1.006x | 0.967x | 3.800x |
| apply_children_full | 1.009x | 0.971x | 1.328x |
| make_unmake_full | 1.000x | 0.957x | 1.312x |
| startpos | 0.963x | 0.963x | 1.254x |
| chess960_materialize | 1.090x | 1.080x | 1.365x |
| chess960_perft_d3 | 1.118x | 1.088x | 1.448x |
| polyglot_key | 1.202x | 1.096x | 38.284x |
| polyglot_recompute | 0.997x | 0.958x | 0.965x |
| repetition_key | 1.036x | 1.017x | 23.151x |
| corpus_count | 1.088x | 0.981x | 1.124x |
| corpus_materialize | 1.036x | 1.019x | 1.055x |
| corpus_uci_sorted | 1.018x | 1.017x | 1.366x |
| replay_trusted_native | 1.058x | 1.037x | 0.716x |
| replay_checked_native | 1.026x | 1.009x | 1.153x |

Cold first legal move list: 0.009 ms -> 0.011 ms (0.85x).

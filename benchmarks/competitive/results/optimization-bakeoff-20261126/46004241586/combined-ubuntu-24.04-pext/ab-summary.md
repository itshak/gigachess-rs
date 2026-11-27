| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.043x | 1.013x | 1.612x |
| materialize | 1.091x | 1.053x | 1.608x |
| perft_bulk_d4 | 1.055x | 1.032x | 1.252x |
| perft_full_d3 | 1.205x | 1.055x | 1.636x |
| fen_parse | 0.935x | 0.827x | 1.043x |
| fen_write_native_ep | 0.994x | 0.966x | 2.100x |
| snapshot | 1.002x | 0.999x | 1.151x |
| san_parse_all | 1.025x | 0.993x | 1.918x |
| san_render_native | 0.983x | 0.975x | 2.151x |
| apply_children_full | 0.997x | 0.987x | 1.334x |
| make_unmake_full | 0.975x | 0.966x | 1.172x |
| startpos | 0.898x | 0.898x | 1.112x |
| chess960_materialize | 0.955x | 0.941x | 1.435x |
| chess960_perft_d3 | 1.016x | 0.995x | 1.488x |
| polyglot_key | 0.996x | 0.991x | 39.526x |
| polyglot_recompute | 0.978x | 0.968x | 0.971x |
| repetition_key | 0.963x | 0.912x | 22.616x |
| corpus_count | 1.161x | 1.114x | 1.303x |
| corpus_materialize | 1.007x | 1.006x | 1.083x |
| corpus_uci_sorted | 1.018x | 1.016x | 1.580x |
| replay_trusted_native | 1.002x | 0.959x | 0.914x |
| replay_checked_native | 1.005x | 0.989x | 1.146x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.20x).

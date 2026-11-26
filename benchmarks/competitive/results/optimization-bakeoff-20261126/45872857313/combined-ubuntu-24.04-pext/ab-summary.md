| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.993x | 1.550x |
| materialize | 0.923x | 0.868x | 1.496x |
| perft_bulk_d4 | 0.995x | 0.980x | 1.186x |
| perft_full_d3 | 1.003x | 0.996x | 1.353x |
| fen_parse | 1.003x | 0.993x | 1.146x |
| fen_write_native_ep | 1.012x | 0.980x | 1.944x |
| snapshot | 1.004x | 0.996x | 1.153x |
| san_parse_all | 1.019x | 0.999x | 1.871x |
| san_render_native | 1.015x | 1.007x | 2.136x |
| apply_children_full | 0.978x | 0.948x | 1.310x |
| make_unmake_full | 0.937x | 0.877x | 1.120x |
| startpos | 1.116x | 1.116x | 1.094x |
| chess960_materialize | 1.005x | 0.987x | 1.509x |
| chess960_perft_d3 | 0.995x | 0.995x | 1.476x |
| polyglot_key | 1.021x | 0.992x | 37.790x |
| polyglot_recompute | 0.971x | 0.968x | 0.974x |
| repetition_key | 0.830x | 0.710x | 19.066x |
| corpus_count | 0.993x | 0.990x | 1.124x |
| corpus_materialize | 1.010x | 1.002x | 1.076x |
| corpus_uci_sorted | 1.015x | 1.008x | 1.569x |
| replay_trusted_native | 1.004x | 0.997x | 0.913x |
| replay_checked_native | 0.982x | 0.973x | 1.162x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.76x).

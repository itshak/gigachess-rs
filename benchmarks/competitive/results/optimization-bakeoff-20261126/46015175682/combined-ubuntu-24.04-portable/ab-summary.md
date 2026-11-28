| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.058x | 1.000x | 1.384x |
| materialize | 1.030x | 1.018x | 1.365x |
| perft_bulk_d4 | 1.064x | 1.038x | 1.339x |
| perft_full_d3 | 1.238x | 1.025x | 1.597x |
| fen_parse | 1.018x | 0.887x | 1.199x |
| fen_write_native_ep | 0.998x | 0.992x | 1.679x |
| snapshot | 1.000x | 0.998x | 0.993x |
| san_parse_all | 1.025x | 1.014x | 2.204x |
| san_render_native | 1.026x | 0.991x | 2.305x |
| apply_children_full | 1.025x | 1.004x | 1.443x |
| make_unmake_full | 1.010x | 0.989x | 1.387x |
| startpos | 1.000x | 1.000x | 1.338x |
| chess960_materialize | 0.987x | 0.964x | 1.625x |
| chess960_perft_d3 | 1.021x | 1.016x | 1.584x |
| polyglot_key | 1.004x | 1.001x | 21.874x |
| polyglot_recompute | 1.224x | 1.067x | 1.192x |
| repetition_key | 0.997x | 0.988x | 20.057x |
| corpus_count | 1.078x | 1.071x | 1.324x |
| corpus_materialize | 1.003x | 1.001x | 1.346x |
| corpus_uci_sorted | 1.012x | 1.009x | 1.609x |
| replay_trusted_native | 1.015x | 1.010x | 1.085x |
| replay_checked_native | 1.002x | 0.999x | 1.156x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.92x).

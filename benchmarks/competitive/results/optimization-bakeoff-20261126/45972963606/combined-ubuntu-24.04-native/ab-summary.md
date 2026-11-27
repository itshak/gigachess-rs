| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.029x | 1.022x | 1.528x |
| materialize | 0.978x | 0.899x | 1.170x |
| perft_bulk_d4 | 1.042x | 0.987x | 1.378x |
| perft_full_d3 | 1.357x | 1.090x | 1.607x |
| fen_parse | 0.782x | 0.759x | 0.898x |
| fen_write_native_ep | 0.959x | 0.907x | 1.768x |
| snapshot | 1.000x | 0.996x | 1.249x |
| san_parse_all | 1.000x | 0.993x | 2.219x |
| san_render_native | 1.033x | 1.010x | 2.191x |
| apply_children_full | 1.020x | 1.003x | 1.326x |
| make_unmake_full | 1.073x | 0.967x | 1.302x |
| startpos | 1.002x | 1.002x | 1.200x |
| chess960_materialize | 1.030x | 1.017x | 1.604x |
| chess960_perft_d3 | 1.010x | 1.004x | 1.622x |
| polyglot_key | 1.002x | 1.001x | 20.687x |
| polyglot_recompute | 1.004x | 1.003x | 1.030x |
| repetition_key | 0.994x | 0.991x | 19.083x |
| corpus_count | 1.065x | 1.037x | 1.389x |
| corpus_materialize | 0.999x | 0.992x | 1.273x |
| corpus_uci_sorted | 0.980x | 0.978x | 1.666x |
| replay_trusted_native | 0.955x | 0.932x | 1.050x |
| replay_checked_native | 0.993x | 0.986x | 1.121x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.19x).

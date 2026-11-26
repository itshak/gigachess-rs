| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.999x | 1.324x |
| materialize | 1.023x | 1.012x | 1.113x |
| perft_bulk_d4 | 0.984x | 0.972x | 1.190x |
| perft_full_d3 | 1.017x | 0.972x | 1.004x |
| fen_parse | 1.010x | 0.987x | 1.222x |
| fen_write_native_ep | 0.999x | 0.989x | 1.836x |
| snapshot | 0.994x | 0.969x | 1.241x |
| san_parse_all | 0.992x | 0.955x | 2.182x |
| san_render_native | 0.986x | 0.929x | 2.096x |
| apply_children_full | 1.015x | 0.982x | 1.201x |
| make_unmake_full | 1.027x | 1.010x | 1.248x |
| startpos | 0.997x | 0.997x | 1.200x |
| chess960_materialize | 0.996x | 0.996x | 1.474x |
| chess960_perft_d3 | 1.009x | 1.004x | 1.516x |
| polyglot_key | 0.991x | 0.981x | 20.592x |
| polyglot_recompute | 1.049x | 1.037x | 1.033x |
| repetition_key | 1.008x | 0.997x | 15.646x |
| corpus_count | 1.003x | 1.001x | 1.186x |
| corpus_materialize | 1.001x | 0.998x | 1.175x |
| corpus_uci_sorted | 0.994x | 0.988x | 1.650x |
| replay_trusted_native | 1.016x | 0.991x | 0.893x |
| replay_checked_native | 0.993x | 0.990x | 1.001x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.04x).

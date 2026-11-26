| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.966x | 1.301x |
| materialize | 0.969x | 0.933x | 1.169x |
| perft_bulk_d4 | 1.020x | 0.989x | 1.192x |
| perft_full_d3 | 0.975x | 0.952x | 1.220x |
| fen_parse | 0.983x | 0.950x | 1.301x |
| fen_write_native_ep | 0.982x | 0.968x | 1.475x |
| snapshot | 0.733x | 0.723x | 1.107x |
| san_parse_all | 0.996x | 0.974x | 1.982x |
| san_render_native | 0.917x | 0.822x | 3.500x |
| apply_children_full | 0.671x | 0.552x | 0.900x |
| make_unmake_full | 0.958x | 0.925x | 1.232x |
| startpos | 1.010x | 1.010x | 1.253x |
| chess960_materialize | 0.898x | 0.878x | 1.269x |
| chess960_perft_d3 | 1.004x | 0.989x | 1.410x |
| polyglot_key | 1.115x | 1.070x | 38.206x |
| polyglot_recompute | 1.026x | 0.995x | 1.014x |
| repetition_key | 1.194x | 1.011x | 26.982x |
| corpus_count | 0.952x | 0.897x | 1.011x |
| corpus_materialize | 0.993x | 0.967x | 1.004x |
| corpus_uci_sorted | 1.009x | 0.997x | 1.393x |
| replay_trusted_native | 1.032x | 1.020x | 0.703x |
| replay_checked_native | 1.017x | 1.009x | 1.146x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.90x).

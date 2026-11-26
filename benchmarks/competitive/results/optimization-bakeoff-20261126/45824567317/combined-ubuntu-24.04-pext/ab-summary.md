| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.983x | 0.888x | 1.376x |
| materialize | 1.005x | 0.997x | 1.179x |
| perft_bulk_d4 | 0.991x | 0.969x | 1.261x |
| perft_full_d3 | 0.983x | 0.944x | 0.974x |
| fen_parse | 1.238x | 1.214x | 1.264x |
| fen_write_native_ep | 1.004x | 0.997x | 1.892x |
| snapshot | 1.001x | 0.998x | 1.251x |
| san_parse_all | 1.004x | 0.997x | 2.289x |
| san_render_native | 1.007x | 0.985x | 2.115x |
| apply_children_full | 1.022x | 0.969x | 1.243x |
| make_unmake_full | 1.037x | 0.994x | 1.219x |
| startpos | 1.000x | 1.000x | 1.203x |
| chess960_materialize | 0.987x | 0.968x | 1.422x |
| chess960_perft_d3 | 0.993x | 0.986x | 1.482x |
| polyglot_key | 1.010x | 1.001x | 20.506x |
| polyglot_recompute | 0.959x | 0.956x | 0.978x |
| repetition_key | 0.998x | 0.996x | 15.493x |
| corpus_count | 1.009x | 1.008x | 1.258x |
| corpus_materialize | 1.000x | 0.999x | 1.227x |
| corpus_uci_sorted | 0.990x | 0.978x | 1.647x |
| replay_trusted_native | 1.060x | 1.023x | 1.162x |
| replay_checked_native | 1.071x | 1.065x | 1.128x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).

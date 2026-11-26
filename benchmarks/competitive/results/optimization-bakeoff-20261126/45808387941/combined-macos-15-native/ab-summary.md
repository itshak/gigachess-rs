| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.887x | 1.316x |
| materialize | 1.069x | 0.855x | 0.925x |
| perft_bulk_d4 | 0.860x | 0.677x | 1.062x |
| perft_full_d3 | 0.932x | 0.665x | 1.269x |
| fen_parse | 0.962x | 0.833x | 1.218x |
| fen_write_native_ep | 1.041x | 0.894x | 1.815x |
| snapshot | 0.995x | 0.676x | 0.924x |
| san_parse_all | 0.914x | 0.810x | 1.741x |
| san_render_native | 1.075x | 0.925x | 2.746x |
| apply_children_full | 1.100x | 0.948x | 1.068x |
| make_unmake_full | 1.089x | 1.033x | 1.313x |
| startpos | 1.235x | 1.235x | 1.179x |
| chess960_materialize | 1.255x | 1.219x | 1.383x |
| chess960_perft_d3 | 0.987x | 0.819x | 1.318x |
| polyglot_key | 1.085x | 0.968x | 39.317x |
| polyglot_recompute | 1.007x | 0.991x | 1.013x |
| repetition_key | 0.941x | 0.931x | 19.144x |
| corpus_count | 0.750x | 0.740x | 1.089x |
| corpus_materialize | 0.965x | 0.883x | 1.113x |
| corpus_uci_sorted | 0.900x | 0.894x | 1.340x |
| replay_trusted_native | 1.105x | 0.958x | 0.589x |
| replay_checked_native | 1.187x | 1.024x | 1.049x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.74x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 1.021x | 1.511x |
| materialize | 1.020x | 1.006x | 1.256x |
| perft_bulk_d4 | 1.033x | 1.017x | 1.414x |
| perft_full_d3 | 1.229x | 1.024x | 1.620x |
| fen_parse | 0.983x | 0.974x | 1.052x |
| fen_write_native_ep | 0.979x | 0.967x | 1.672x |
| snapshot | 1.000x | 0.995x | 1.250x |
| san_parse_all | 0.989x | 0.951x | 1.993x |
| san_render_native | 0.993x | 0.985x | 2.170x |
| apply_children_full | 1.003x | 0.992x | 1.327x |
| make_unmake_full | 1.020x | 0.979x | 1.286x |
| startpos | 0.999x | 0.999x | 1.255x |
| chess960_materialize | 0.995x | 0.970x | 1.385x |
| chess960_perft_d3 | 1.004x | 0.981x | 1.519x |
| polyglot_key | 0.999x | 0.997x | 27.693x |
| polyglot_recompute | 0.813x | 0.812x | 0.840x |
| repetition_key | 0.976x | 0.971x | 19.926x |
| corpus_count | 1.065x | 1.044x | 1.447x |
| corpus_materialize | 1.015x | 1.011x | 1.163x |
| corpus_uci_sorted | 0.991x | 0.984x | 1.717x |
| replay_trusted_native | 0.962x | 0.962x | 0.881x |
| replay_checked_native | 1.000x | 0.987x | 1.058x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.97x).

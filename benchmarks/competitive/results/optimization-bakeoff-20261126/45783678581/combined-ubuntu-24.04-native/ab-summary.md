| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.988x | 1.408x |
| materialize | 0.951x | 0.935x | 1.453x |
| perft_bulk_d4 | 0.999x | 0.990x | 1.213x |
| perft_full_d3 | 0.989x | 0.972x | 1.541x |
| fen_parse | 1.150x | 1.109x | 1.113x |
| fen_write_native_ep | 0.977x | 0.956x | 1.342x |
| snapshot | 1.409x | 1.404x | 1.151x |
| san_parse_all | 0.987x | 0.979x | 1.853x |
| san_render_native | 0.986x | 0.982x | 2.136x |
| apply_children_full | 1.107x | 1.008x | 1.222x |
| make_unmake_full | 0.993x | 0.965x | 1.144x |
| startpos | 1.223x | 1.223x | 1.120x |
| chess960_materialize | 0.996x | 0.994x | 1.440x |
| chess960_perft_d3 | 0.993x | 0.990x | 1.379x |
| polyglot_key | 0.993x | 0.988x | 39.417x |
| polyglot_recompute | 1.031x | 1.024x | 1.011x |
| repetition_key | 1.008x | 0.976x | 19.189x |
| corpus_count | 1.003x | 0.997x | 1.346x |
| corpus_materialize | 1.017x | 1.011x | 1.103x |
| corpus_uci_sorted | 1.005x | 1.001x | 1.582x |
| replay_trusted_native | 1.070x | 0.971x | 0.775x |
| replay_checked_native | 0.996x | 0.984x | 1.122x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.97x).

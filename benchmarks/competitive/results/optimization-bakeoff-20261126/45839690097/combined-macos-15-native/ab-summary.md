| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.924x | 0.773x | 1.464x |
| materialize | 1.033x | 0.864x | 1.175x |
| perft_bulk_d4 | 0.932x | 0.772x | 1.229x |
| perft_full_d3 | 1.117x | 0.808x | 1.247x |
| fen_parse | 1.172x | 1.011x | 1.233x |
| fen_write_native_ep | 0.962x | 0.797x | 1.691x |
| snapshot | 1.158x | 0.713x | 0.950x |
| san_parse_all | 0.963x | 0.887x | 1.669x |
| san_render_native | 1.049x | 0.860x | 2.835x |
| apply_children_full | 0.925x | 0.725x | 1.044x |
| make_unmake_full | 1.022x | 0.800x | 1.359x |
| startpos | 0.716x | 0.716x | 0.758x |
| chess960_materialize | 0.920x | 0.889x | 1.542x |
| chess960_perft_d3 | 0.753x | 0.590x | 1.155x |
| polyglot_key | 0.835x | 0.793x | 39.478x |
| polyglot_recompute | 0.930x | 0.866x | 1.050x |
| repetition_key | 0.837x | 0.597x | 20.354x |
| corpus_count | 0.960x | 0.910x | 1.393x |
| corpus_materialize | 0.991x | 0.917x | 1.173x |
| corpus_uci_sorted | 0.766x | 0.725x | 1.128x |
| replay_trusted_native | 0.903x | 0.807x | 0.687x |
| replay_checked_native | 0.813x | 0.779x | 1.244x |

Cold first legal move list: 0.005 ms -> 0.004 ms (1.43x).

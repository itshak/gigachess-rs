| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.355x |
| materialize | 0.988x | 0.983x | 1.228x |
| perft_bulk_d4 | 1.003x | 1.000x | 1.353x |
| perft_full_d3 | 0.990x | 0.978x | 1.045x |
| fen_parse | 1.008x | 1.003x | 1.257x |
| fen_write_native_ep | 0.999x | 0.997x | 1.582x |
| snapshot | 0.911x | 0.902x | 0.894x |
| san_parse_all | 0.998x | 0.994x | 1.833x |
| san_render_native | 0.999x | 0.994x | 2.558x |
| apply_children_full | 0.975x | 0.952x | 1.012x |
| make_unmake_full | 0.993x | 0.982x | 1.433x |
| startpos | 0.997x | 0.997x | 1.187x |
| chess960_materialize | 0.995x | 0.993x | 1.317x |
| chess960_perft_d3 | 1.001x | 1.000x | 1.331x |
| polyglot_key | 1.006x | 1.001x | 25.824x |
| polyglot_recompute | 0.989x | 0.985x | 1.065x |
| repetition_key | 1.001x | 1.000x | 19.359x |
| corpus_count | 0.997x | 0.993x | 1.464x |
| corpus_materialize | 0.991x | 0.990x | 1.191x |
| corpus_uci_sorted | 1.002x | 1.000x | 1.459x |
| replay_trusted_native | 1.002x | 1.001x | 0.506x |
| replay_checked_native | 0.982x | 0.980x | 0.981x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.09x).

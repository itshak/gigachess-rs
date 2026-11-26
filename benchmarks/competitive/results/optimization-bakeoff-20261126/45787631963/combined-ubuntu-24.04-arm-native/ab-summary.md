| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 0.940x | 1.379x |
| materialize | 1.012x | 0.978x | 1.255x |
| perft_bulk_d4 | 0.918x | 0.895x | 1.241x |
| perft_full_d3 | 0.991x | 0.985x | 1.044x |
| fen_parse | 0.990x | 0.986x | 1.242x |
| fen_write_native_ep | 1.001x | 0.999x | 1.579x |
| snapshot | 1.000x | 0.998x | 1.031x |
| san_parse_all | 0.999x | 0.995x | 1.834x |
| san_render_native | 0.996x | 0.990x | 2.525x |
| apply_children_full | 1.002x | 0.998x | 1.025x |
| make_unmake_full | 1.000x | 0.993x | 1.436x |
| startpos | 1.000x | 1.000x | 1.157x |
| chess960_materialize | 1.050x | 0.998x | 1.397x |
| chess960_perft_d3 | 1.014x | 0.960x | 1.350x |
| polyglot_key | 1.023x | 1.002x | 25.791x |
| polyglot_recompute | 0.999x | 0.998x | 1.070x |
| repetition_key | 1.007x | 0.999x | 19.243x |
| corpus_count | 0.826x | 0.794x | 1.195x |
| corpus_materialize | 0.972x | 0.962x | 1.165x |
| corpus_uci_sorted | 1.008x | 1.001x | 1.455x |
| replay_trusted_native | 0.979x | 0.957x | 0.487x |
| replay_checked_native | 1.004x | 1.003x | 0.989x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.73x).

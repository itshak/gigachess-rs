| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.972x | 1.313x |
| materialize | 1.016x | 0.990x | 1.097x |
| perft_bulk_d4 | 0.985x | 0.963x | 1.236x |
| perft_full_d3 | 0.820x | 0.778x | 0.774x |
| fen_parse | 0.981x | 0.951x | 1.096x |
| fen_write_native_ep | 0.976x | 0.947x | 1.192x |
| snapshot | 3.034x | 3.006x | 1.105x |
| san_parse_all | 0.998x | 0.985x | 2.123x |
| san_render_native | 1.004x | 0.977x | 2.194x |
| apply_children_full | 0.978x | 0.934x | 1.042x |
| make_unmake_full | 0.989x | 0.971x | 1.195x |
| startpos | 1.095x | 1.095x | 1.182x |
| chess960_materialize | 0.989x | 0.981x | 1.430x |
| chess960_perft_d3 | 0.984x | 0.982x | 1.399x |
| polyglot_key | 1.005x | 1.000x | 20.579x |
| polyglot_recompute | 1.010x | 0.998x | 1.022x |
| repetition_key | 0.995x | 0.990x | 19.183x |
| corpus_count | 1.003x | 1.002x | 1.321x |
| corpus_materialize | 0.999x | 0.993x | 1.221x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.669x |
| replay_trusted_native | 0.997x | 0.972x | 0.763x |
| replay_checked_native | 0.991x | 0.982x | 0.938x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.02x).

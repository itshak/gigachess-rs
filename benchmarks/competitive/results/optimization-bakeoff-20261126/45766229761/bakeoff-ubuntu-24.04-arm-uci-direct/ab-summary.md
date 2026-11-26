| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.084x |
| materialize | 1.006x | 1.001x | 1.176x |
| perft_bulk_d4 | 1.010x | 1.006x | 0.883x |
| perft_full_d3 | 1.001x | 1.000x | 0.597x |
| fen_parse | 1.008x | 1.002x | 1.211x |
| fen_write_native_ep | 1.006x | 0.997x | 1.060x |
| snapshot | 1.024x | 1.006x | 0.818x |
| san_parse_all | 1.000x | 0.993x | 0.913x |
| san_render_native | 1.008x | 1.002x | 1.149x |
| apply_children_full | 0.998x | 0.982x | 0.655x |
| make_unmake_full | 0.999x | 0.986x | 0.892x |
| startpos | 1.002x | 1.002x | 0.880x |
| chess960_materialize | 1.002x | 1.000x | 1.114x |
| chess960_perft_d3 | 1.000x | 0.998x | 0.845x |
| polyglot_key | 0.989x | 0.986x | 22.805x |
| polyglot_recompute | 1.003x | 0.997x | 1.133x |
| repetition_key | 0.994x | 0.987x | 13.869x |
| corpus_count | 1.021x | 1.012x | 1.224x |
| corpus_materialize | 0.995x | 0.993x | 1.118x |
| corpus_uci_sorted | 1.695x | 1.694x | 0.964x |
| replay_trusted_native | 1.010x | 1.007x | 0.324x |
| replay_checked_native | 1.012x | 1.008x | 0.659x |

Cold first legal move list: 154.124 ms → 154.025 ms (1.00x).

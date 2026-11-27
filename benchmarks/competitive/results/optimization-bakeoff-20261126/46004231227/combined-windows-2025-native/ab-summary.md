| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.052x | 0.999x | 1.544x |
| materialize | 1.028x | 0.996x | 1.138x |
| perft_bulk_d4 | 0.997x | 0.970x | 1.445x |
| perft_full_d3 | 1.013x | 0.991x | 1.298x |
| fen_parse | 1.108x | 1.050x | 1.536x |
| fen_write_native_ep | 1.007x | 0.994x | 1.600x |
| snapshot | 0.996x | 0.966x | 1.170x |
| san_parse_all | 1.030x | 0.986x | 1.993x |
| san_render_native | 1.004x | 0.985x | 3.250x |
| apply_children_full | 1.007x | 0.977x | 1.320x |
| make_unmake_full | 1.026x | 0.942x | 1.289x |
| startpos | 1.002x | 1.002x | 1.001x |
| chess960_materialize | 1.000x | 0.999x | 1.360x |
| chess960_perft_d3 | 0.991x | 0.982x | 1.589x |
| polyglot_key | 0.998x | 0.995x | 20.323x |
| polyglot_recompute | 1.015x | 1.003x | 0.991x |
| repetition_key | 0.996x | 0.993x | 18.893x |
| corpus_count | 0.990x | 0.987x | 1.283x |
| corpus_materialize | 1.016x | 1.012x | 1.195x |
| corpus_uci_sorted | 1.004x | 0.994x | 1.501x |
| replay_trusted_native | 1.036x | 1.012x | 1.118x |
| replay_checked_native | 0.993x | 0.991x | 1.086x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.09x).

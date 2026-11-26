| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.968x | 0.913x | 1.365x |
| materialize | 0.935x | 0.849x | 1.437x |
| perft_bulk_d4 | 0.840x | 0.749x | 1.017x |
| perft_full_d3 | 1.018x | 0.934x | 1.409x |
| fen_parse | 0.995x | 0.963x | 1.055x |
| fen_write_native_ep | 1.000x | 0.953x | 2.115x |
| snapshot | 1.666x | 1.662x | 1.000x |
| san_parse_all | 0.971x | 0.957x | 2.038x |
| san_render_native | 1.040x | 1.022x | 2.230x |
| apply_children_full | 1.030x | 0.939x | 1.175x |
| make_unmake_full | 1.003x | 0.967x | 1.119x |
| startpos | 1.498x | 1.498x | 1.246x |
| chess960_materialize | 0.986x | 0.923x | 1.384x |
| chess960_perft_d3 | 1.027x | 0.986x | 1.417x |
| polyglot_key | 0.930x | 0.865x | 37.419x |
| polyglot_recompute | 1.044x | 1.037x | 1.065x |
| repetition_key | 0.988x | 0.803x | 19.102x |
| corpus_count | 0.640x | 0.534x | 0.845x |
| corpus_materialize | 0.943x | 0.925x | 1.027x |
| corpus_uci_sorted | 1.006x | 1.004x | 1.582x |
| replay_trusted_native | 1.006x | 0.977x | 0.834x |
| replay_checked_native | 0.991x | 0.990x | 1.155x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.74x).

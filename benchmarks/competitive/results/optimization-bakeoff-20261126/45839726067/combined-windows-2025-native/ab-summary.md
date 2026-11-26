| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.970x | 1.494x |
| materialize | 1.056x | 1.022x | 1.258x |
| perft_bulk_d4 | 1.037x | 0.975x | 1.354x |
| perft_full_d3 | 1.062x | 0.934x | 1.084x |
| fen_parse | 1.335x | 1.221x | 1.439x |
| fen_write_native_ep | 1.058x | 1.006x | 1.572x |
| snapshot | 1.033x | 0.987x | 0.999x |
| san_parse_all | 1.009x | 0.992x | 2.050x |
| san_render_native | 1.072x | 1.054x | 3.962x |
| apply_children_full | 1.065x | 0.995x | 1.399x |
| make_unmake_full | 1.009x | 0.967x | 1.348x |
| startpos | 1.000x | 1.000x | 1.247x |
| chess960_materialize | 1.238x | 1.189x | 1.359x |
| chess960_perft_d3 | 1.018x | 0.986x | 1.541x |
| polyglot_key | 1.039x | 0.943x | 28.672x |
| polyglot_recompute | 1.106x | 1.093x | 1.063x |
| repetition_key | 1.012x | 1.006x | 22.128x |
| corpus_count | 1.062x | 1.016x | 1.336x |
| corpus_materialize | 1.110x | 1.102x | 1.148x |
| corpus_uci_sorted | 1.035x | 0.997x | 1.390x |
| replay_trusted_native | 0.993x | 0.989x | 0.816x |
| replay_checked_native | 1.210x | 1.142x | 1.107x |

Cold first legal move list: 0.027 ms -> 0.018 ms (1.52x).

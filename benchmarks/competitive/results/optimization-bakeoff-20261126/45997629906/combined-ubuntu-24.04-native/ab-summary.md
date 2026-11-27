| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.949x | 1.430x |
| materialize | 0.990x | 0.966x | 1.208x |
| perft_bulk_d4 | 1.006x | 0.989x | 1.268x |
| perft_full_d3 | 1.067x | 1.022x | 1.240x |
| fen_parse | 0.933x | 0.880x | 1.107x |
| fen_write_native_ep | 0.999x | 0.976x | 1.550x |
| snapshot | 1.009x | 0.991x | 1.500x |
| san_parse_all | 1.000x | 0.989x | 1.883x |
| san_render_native | 0.982x | 0.960x | 2.313x |
| apply_children_full | 0.986x | 0.952x | 1.350x |
| make_unmake_full | 0.921x | 0.829x | 1.108x |
| startpos | 0.985x | 0.985x | 1.251x |
| chess960_materialize | 1.027x | 1.010x | 1.538x |
| chess960_perft_d3 | 1.013x | 1.012x | 1.507x |
| polyglot_key | 1.019x | 1.006x | 35.784x |
| polyglot_recompute | 0.990x | 0.987x | 0.988x |
| repetition_key | 1.005x | 1.003x | 27.923x |
| corpus_count | 0.992x | 0.985x | 1.148x |
| corpus_materialize | 1.019x | 1.007x | 1.149x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.697x |
| replay_trusted_native | 1.038x | 0.969x | 0.790x |
| replay_checked_native | 0.985x | 0.948x | 1.154x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.96x).

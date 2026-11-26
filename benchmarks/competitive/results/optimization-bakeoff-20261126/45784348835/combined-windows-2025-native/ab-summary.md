| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.985x | 1.308x |
| materialize | 1.031x | 1.017x | 1.195x |
| perft_bulk_d4 | 1.001x | 0.980x | 1.344x |
| perft_full_d3 | 0.977x | 0.946x | 1.208x |
| fen_parse | 0.996x | 0.965x | 1.223x |
| fen_write_native_ep | 0.993x | 0.964x | 1.580x |
| snapshot | 0.997x | 0.986x | 1.036x |
| san_parse_all | 0.998x | 0.995x | 1.962x |
| san_render_native | 0.997x | 0.970x | 3.790x |
| apply_children_full | 1.085x | 1.062x | 1.272x |
| make_unmake_full | 1.015x | 1.001x | 1.278x |
| startpos | 1.152x | 1.152x | 1.106x |
| chess960_materialize | 1.013x | 1.001x | 1.251x |
| chess960_perft_d3 | 1.005x | 0.988x | 1.338x |
| polyglot_key | 1.000x | 0.999x | 24.915x |
| polyglot_recompute | 1.001x | 0.999x | 1.022x |
| repetition_key | 1.035x | 1.020x | 19.106x |
| corpus_count | 1.010x | 1.002x | 1.369x |
| corpus_materialize | 1.000x | 0.994x | 1.172x |
| corpus_uci_sorted | 1.002x | 1.001x | 1.424x |
| replay_trusted_native | 1.060x | 1.009x | 0.729x |
| replay_checked_native | 0.999x | 0.991x | 0.986x |

Cold first legal move list: 0.032 ms -> 0.026 ms (1.25x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.968x | 0.867x | 1.503x |
| materialize | 0.981x | 0.965x | 1.145x |
| perft_bulk_d4 | 0.995x | 0.966x | 1.440x |
| perft_full_d3 | 1.260x | 1.050x | 1.625x |
| fen_parse | 0.911x | 0.825x | 1.309x |
| fen_write_native_ep | 0.999x | 0.995x | 1.658x |
| snapshot | 0.950x | 0.851x | 1.129x |
| san_parse_all | 0.986x | 0.946x | 1.999x |
| san_render_native | 1.095x | 1.034x | 3.713x |
| apply_children_full | 1.015x | 0.966x | 1.336x |
| make_unmake_full | 1.028x | 0.986x | 1.357x |
| startpos | 1.001x | 1.001x | 1.000x |
| chess960_materialize | 1.015x | 1.001x | 1.354x |
| chess960_perft_d3 | 1.006x | 0.999x | 1.619x |
| polyglot_key | 1.003x | 1.003x | 20.188x |
| polyglot_recompute | 1.094x | 0.998x | 1.070x |
| repetition_key | 1.002x | 0.998x | 19.264x |
| corpus_count | 1.001x | 0.997x | 1.298x |
| corpus_materialize | 1.007x | 1.002x | 1.187x |
| corpus_uci_sorted | 1.002x | 0.997x | 1.490x |
| replay_trusted_native | 0.854x | 0.719x | 0.909x |
| replay_checked_native | 0.952x | 0.923x | 1.034x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.04x).

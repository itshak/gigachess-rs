| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.999x | 1.501x |
| materialize | 0.987x | 0.890x | 1.489x |
| perft_bulk_d4 | 0.986x | 0.978x | 1.146x |
| perft_full_d3 | 1.000x | 0.992x | 1.393x |
| fen_parse | 0.991x | 0.982x | 1.168x |
| fen_write_native_ep | 0.996x | 0.973x | 2.120x |
| snapshot | 1.000x | 0.999x | 1.001x |
| san_parse_all | 1.067x | 0.991x | 1.855x |
| san_render_native | 0.967x | 0.887x | 2.045x |
| apply_children_full | 0.960x | 0.931x | 1.270x |
| make_unmake_full | 0.912x | 0.861x | 1.077x |
| startpos | 0.893x | 0.893x | 1.000x |
| chess960_materialize | 0.966x | 0.941x | 1.312x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.497x |
| polyglot_key | 1.011x | 1.004x | 39.576x |
| polyglot_recompute | 0.999x | 0.993x | 1.010x |
| repetition_key | 0.801x | 0.761x | 24.095x |
| corpus_count | 0.989x | 0.985x | 0.948x |
| corpus_materialize | 0.993x | 0.992x | 1.032x |
| corpus_uci_sorted | 1.015x | 1.013x | 1.562x |
| replay_trusted_native | 1.023x | 1.016x | 0.873x |
| replay_checked_native | 0.967x | 0.930x | 1.037x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).

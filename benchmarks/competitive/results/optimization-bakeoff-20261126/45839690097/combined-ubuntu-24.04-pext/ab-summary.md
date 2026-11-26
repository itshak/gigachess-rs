| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.995x | 1.565x |
| materialize | 0.985x | 0.967x | 1.594x |
| perft_bulk_d4 | 1.002x | 0.987x | 1.196x |
| perft_full_d3 | 1.039x | 1.005x | 1.335x |
| fen_parse | 1.062x | 0.991x | 1.153x |
| fen_write_native_ep | 0.994x | 0.969x | 1.917x |
| snapshot | 0.997x | 0.990x | 1.000x |
| san_parse_all | 0.952x | 0.928x | 1.813x |
| san_render_native | 0.969x | 0.957x | 2.081x |
| apply_children_full | 0.996x | 0.994x | 1.315x |
| make_unmake_full | 0.999x | 0.979x | 1.172x |
| startpos | 0.802x | 0.802x | 1.002x |
| chess960_materialize | 0.988x | 0.986x | 1.512x |
| chess960_perft_d3 | 1.001x | 0.996x | 1.471x |
| polyglot_key | 0.984x | 0.970x | 39.011x |
| polyglot_recompute | 1.059x | 1.050x | 1.028x |
| repetition_key | 0.972x | 0.951x | 18.840x |
| corpus_count | 1.000x | 0.998x | 1.069x |
| corpus_materialize | 1.002x | 1.002x | 1.086x |
| corpus_uci_sorted | 0.993x | 0.986x | 1.576x |
| replay_trusted_native | 0.978x | 0.972x | 0.869x |
| replay_checked_native | 1.010x | 0.994x | 1.098x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.04x).

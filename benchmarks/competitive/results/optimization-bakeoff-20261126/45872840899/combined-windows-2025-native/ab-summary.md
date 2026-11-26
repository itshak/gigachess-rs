| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.942x | 0.795x | 1.400x |
| materialize | 1.001x | 0.976x | 1.164x |
| perft_bulk_d4 | 1.016x | 0.967x | 1.347x |
| perft_full_d3 | 1.019x | 0.994x | 1.304x |
| fen_parse | 0.872x | 0.825x | 1.194x |
| fen_write_native_ep | 1.001x | 0.983x | 1.533x |
| snapshot | 1.002x | 0.747x | 1.432x |
| san_parse_all | 0.972x | 0.828x | 1.899x |
| san_render_native | 0.997x | 0.931x | 4.020x |
| apply_children_full | 1.006x | 0.960x | 1.413x |
| make_unmake_full | 0.995x | 0.855x | 1.347x |
| startpos | 0.985x | 0.985x | 0.997x |
| chess960_materialize | 1.010x | 0.989x | 1.441x |
| chess960_perft_d3 | 1.012x | 0.993x | 1.504x |
| polyglot_key | 0.980x | 0.963x | 35.738x |
| polyglot_recompute | 1.003x | 0.991x | 0.996x |
| repetition_key | 1.002x | 1.001x | 28.264x |
| corpus_count | 0.985x | 0.977x | 1.153x |
| corpus_materialize | 0.979x | 0.970x | 1.084x |
| corpus_uci_sorted | 1.001x | 0.991x | 1.391x |
| replay_trusted_native | 1.026x | 0.929x | 0.747x |
| replay_checked_native | 0.932x | 0.893x | 1.106x |

Cold first legal move list: 0.017 ms -> 0.017 ms (0.99x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.913x | 1.415x |
| materialize | 0.995x | 0.986x | 1.174x |
| perft_bulk_d4 | 1.009x | 0.984x | 1.329x |
| perft_full_d3 | 1.023x | 0.992x | 1.007x |
| fen_parse | 1.013x | 0.982x | 1.204x |
| fen_write_native_ep | 1.000x | 0.991x | 1.891x |
| snapshot | 0.852x | 0.836x | 0.840x |
| san_parse_all | 0.990x | 0.983x | 2.182x |
| san_render_native | 0.995x | 0.908x | 2.126x |
| apply_children_full | 1.024x | 1.012x | 1.343x |
| make_unmake_full | 1.013x | 1.001x | 1.209x |
| startpos | 0.998x | 0.998x | 1.165x |
| chess960_materialize | 0.987x | 0.984x | 1.471x |
| chess960_perft_d3 | 1.005x | 1.004x | 1.489x |
| polyglot_key | 1.014x | 1.000x | 20.619x |
| polyglot_recompute | 1.044x | 1.043x | 1.022x |
| repetition_key | 0.999x | 0.992x | 18.946x |
| corpus_count | 1.002x | 1.000x | 1.475x |
| corpus_materialize | 1.008x | 1.006x | 1.264x |
| corpus_uci_sorted | 1.013x | 1.007x | 1.692x |
| replay_trusted_native | 0.998x | 0.995x | 0.740x |
| replay_checked_native | 0.990x | 0.984x | 0.972x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.00x).

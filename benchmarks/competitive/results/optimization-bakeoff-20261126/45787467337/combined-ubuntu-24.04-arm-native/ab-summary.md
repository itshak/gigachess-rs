| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.355x |
| materialize | 0.998x | 0.993x | 1.247x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.350x |
| perft_full_d3 | 1.001x | 0.997x | 1.055x |
| fen_parse | 1.002x | 0.997x | 1.239x |
| fen_write_native_ep | 1.000x | 0.999x | 1.571x |
| snapshot | 0.997x | 0.995x | 1.030x |
| san_parse_all | 0.999x | 0.994x | 1.811x |
| san_render_native | 1.000x | 0.993x | 2.536x |
| apply_children_full | 0.992x | 0.979x | 1.025x |
| make_unmake_full | 1.016x | 0.999x | 1.473x |
| startpos | 0.933x | 0.933x | 1.162x |
| chess960_materialize | 1.004x | 1.003x | 1.335x |
| chess960_perft_d3 | 1.000x | 0.999x | 1.327x |
| polyglot_key | 0.991x | 0.989x | 25.212x |
| polyglot_recompute | 0.997x | 0.997x | 1.070x |
| repetition_key | 0.995x | 0.990x | 18.754x |
| corpus_count | 0.997x | 0.994x | 1.486x |
| corpus_materialize | 1.005x | 1.004x | 1.226x |
| corpus_uci_sorted | 1.003x | 1.001x | 1.462x |
| replay_trusted_native | 1.045x | 1.044x | 0.526x |
| replay_checked_native | 1.026x | 1.023x | 1.013x |

Cold first legal move list: 0.012 ms -> 0.012 ms (0.99x).

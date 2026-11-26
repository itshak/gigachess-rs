| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.986x | 1.308x |
| materialize | 1.012x | 0.999x | 1.660x |
| perft_bulk_d4 | 0.980x | 0.903x | 1.169x |
| perft_full_d3 | 1.224x | 1.062x | 1.616x |
| fen_parse | 1.023x | 0.990x | 1.175x |
| fen_write_native_ep | 1.007x | 0.994x | 1.927x |
| snapshot | 1.002x | 0.978x | 1.002x |
| san_parse_all | 1.004x | 0.970x | 1.808x |
| san_render_native | 0.964x | 0.940x | 2.130x |
| apply_children_full | 1.000x | 0.997x | 1.449x |
| make_unmake_full | 0.999x | 0.986x | 1.257x |
| startpos | 1.001x | 1.001x | 1.429x |
| chess960_materialize | 1.029x | 1.003x | 1.477x |
| chess960_perft_d3 | 1.004x | 1.002x | 1.494x |
| polyglot_key | 0.992x | 0.985x | 41.908x |
| polyglot_recompute | 0.997x | 0.992x | 0.980x |
| repetition_key | 1.231x | 1.188x | 32.102x |
| corpus_count | 0.995x | 0.990x | 1.029x |
| corpus_materialize | 1.000x | 0.998x | 1.125x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.554x |
| replay_trusted_native | 0.961x | 0.948x | 0.829x |
| replay_checked_native | 1.000x | 0.997x | 1.102x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.93x).

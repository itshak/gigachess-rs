| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.993x | 1.556x |
| materialize | 1.007x | 0.999x | 1.653x |
| perft_bulk_d4 | 1.003x | 0.992x | 1.189x |
| perft_full_d3 | 1.002x | 1.000x | 1.343x |
| fen_parse | 0.999x | 0.971x | 1.098x |
| fen_write_native_ep | 0.990x | 0.947x | 1.857x |
| snapshot | 1.001x | 0.992x | 1.151x |
| san_parse_all | 0.916x | 0.898x | 1.687x |
| san_render_native | 1.007x | 0.996x | 2.110x |
| apply_children_full | 1.017x | 0.999x | 1.362x |
| make_unmake_full | 0.974x | 0.915x | 1.161x |
| startpos | 1.012x | 1.012x | 1.119x |
| chess960_materialize | 1.126x | 1.122x | 1.520x |
| chess960_perft_d3 | 0.991x | 0.987x | 1.465x |
| polyglot_key | 1.030x | 1.004x | 36.736x |
| polyglot_recompute | 1.025x | 1.021x | 1.013x |
| repetition_key | 1.036x | 1.012x | 23.026x |
| corpus_count | 1.004x | 0.998x | 1.123x |
| corpus_materialize | 1.037x | 1.037x | 1.080x |
| corpus_uci_sorted | 1.007x | 1.006x | 1.539x |
| replay_trusted_native | 0.935x | 0.870x | 0.829x |
| replay_checked_native | 0.981x | 0.979x | 1.170x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.93x).

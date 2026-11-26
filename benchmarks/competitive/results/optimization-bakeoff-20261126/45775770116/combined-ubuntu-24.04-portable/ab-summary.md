| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.149x |
| materialize | 1.068x | 1.040x | 1.661x |
| perft_bulk_d4 | 1.004x | 0.990x | 1.142x |
| perft_full_d3 | 0.999x | 0.990x | 0.931x |
| fen_parse | 0.987x | 0.950x | 1.155x |
| fen_write_native_ep | 1.008x | 0.981x | 1.260x |
| snapshot | 1.118x | 1.112x | 1.000x |
| san_parse_all | 0.988x | 0.974x | 1.667x |
| san_render_native | 1.013x | 0.998x | 2.152x |
| apply_children_full | 1.030x | 0.994x | 1.059x |
| make_unmake_full | 1.031x | 0.998x | 1.006x |
| startpos | 1.001x | 1.001x | 1.357x |
| chess960_materialize | 1.000x | 0.999x | 1.342x |
| chess960_perft_d3 | 0.991x | 0.989x | 1.271x |
| polyglot_key | 0.997x | 0.994x | 40.059x |
| polyglot_recompute | 1.048x | 1.039x | 1.013x |
| repetition_key | 0.986x | 0.982x | 22.064x |
| corpus_count | 1.002x | 1.002x | 1.233x |
| corpus_materialize | 0.995x | 0.994x | 1.142x |
| corpus_uci_sorted | 0.997x | 0.992x | 0.971x |
| replay_trusted_native | 1.048x | 1.020x | 0.622x |
| replay_checked_native | 1.027x | 1.002x | 0.962x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.88x).

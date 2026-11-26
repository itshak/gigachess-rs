| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.083x |
| materialize | 1.004x | 1.000x | 1.189x |
| perft_bulk_d4 | 1.051x | 1.033x | 0.938x |
| perft_full_d3 | 0.998x | 0.997x | 0.598x |
| fen_parse | 0.988x | 0.982x | 1.184x |
| fen_write_native_ep | 1.003x | 1.000x | 1.071x |
| snapshot | 1.030x | 1.015x | 0.758x |
| san_parse_all | 0.998x | 0.990x | 0.929x |
| san_render_native | 1.006x | 0.996x | 1.148x |
| apply_children_full | 0.991x | 0.983x | 0.651x |
| make_unmake_full | 0.998x | 0.994x | 0.894x |
| startpos | 0.988x | 0.988x | 0.806x |
| chess960_materialize | 1.000x | 0.999x | 1.110x |
| chess960_perft_d3 | 1.035x | 1.017x | 0.870x |
| polyglot_key | 0.999x | 0.997x | 23.444x |
| polyglot_recompute | 1.004x | 0.994x | 1.133x |
| repetition_key | 1.010x | 0.997x | 14.188x |
| corpus_count | 0.970x | 0.946x | 1.161x |
| corpus_materialize | 0.990x | 0.985x | 1.117x |
| corpus_uci_sorted | 1.001x | 1.001x | 0.570x |
| replay_trusted_native | 1.000x | 0.994x | 0.316x |
| replay_checked_native | 1.009x | 1.000x | 0.669x |

Cold first legal move list: 154.351 ms → 153.997 ms (1.00x).

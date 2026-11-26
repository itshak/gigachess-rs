| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.559x |
| materialize | 1.010x | 1.007x | 1.322x |
| perft_bulk_d4 | 1.003x | 1.000x | 1.398x |
| perft_full_d3 | 1.005x | 1.000x | 1.333x |
| fen_parse | 0.995x | 0.991x | 1.214x |
| fen_write_native_ep | 0.998x | 0.991x | 1.559x |
| snapshot | 0.982x | 0.978x | 0.957x |
| san_parse_all | 0.999x | 0.993x | 1.828x |
| san_render_native | 1.003x | 0.997x | 2.685x |
| apply_children_full | 0.993x | 0.977x | 1.149x |
| make_unmake_full | 0.995x | 0.941x | 1.555x |
| startpos | 1.001x | 1.001x | 1.333x |
| chess960_materialize | 1.003x | 0.994x | 1.514x |
| chess960_perft_d3 | 0.997x | 0.993x | 1.500x |
| polyglot_key | 1.011x | 0.999x | 26.021x |
| polyglot_recompute | 0.994x | 0.994x | 1.083x |
| repetition_key | 1.002x | 0.998x | 19.549x |
| corpus_count | 1.006x | 1.002x | 1.355x |
| corpus_materialize | 0.994x | 0.990x | 1.221x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.455x |
| replay_trusted_native | 0.948x | 0.924x | 0.605x |
| replay_checked_native | 1.011x | 1.004x | 1.142x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.89x).

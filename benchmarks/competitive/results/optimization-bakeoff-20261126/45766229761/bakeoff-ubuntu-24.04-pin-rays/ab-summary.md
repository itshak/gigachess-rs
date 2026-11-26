| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.174x | 1.086x | 1.397x |
| materialize | 1.038x | 1.010x | 1.466x |
| perft_bulk_d4 | 1.086x | 1.069x | 0.951x |
| perft_full_d3 | 1.003x | 0.996x | 0.598x |
| fen_parse | 0.932x | 0.900x | 1.034x |
| fen_write_native_ep | 0.986x | 0.960x | 1.212x |
| snapshot | 1.563x | 1.557x | 0.794x |
| san_parse_all | 1.010x | 0.998x | 0.910x |
| san_render_native | 1.003x | 0.995x | 1.093x |
| apply_children_full | 1.066x | 1.054x | 0.650x |
| make_unmake_full | 0.997x | 0.967x | 0.766x |
| startpos | 1.007x | 1.007x | 0.559x |
| chess960_materialize | 1.043x | 1.025x | 1.210x |
| chess960_perft_d3 | 1.047x | 1.029x | 0.994x |
| polyglot_key | 1.012x | 1.000x | 36.753x |
| polyglot_recompute | 0.976x | 0.975x | 0.983x |
| repetition_key | 1.024x | 1.002x | 14.508x |
| corpus_count | 1.081x | 1.014x | 1.240x |
| corpus_materialize | 0.993x | 0.989x | 1.068x |
| corpus_uci_sorted | 0.971x | 0.970x | 0.603x |
| replay_trusted_native | 1.030x | 1.020x | 0.518x |
| replay_checked_native | 1.002x | 0.983x | 0.750x |

Cold first legal move list: 139.373 ms → 163.500 ms (0.85x).

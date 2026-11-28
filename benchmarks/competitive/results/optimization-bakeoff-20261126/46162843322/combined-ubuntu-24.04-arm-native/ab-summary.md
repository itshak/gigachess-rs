| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.982x | 1.568x |
| materialize | 1.008x | 0.994x | 1.336x |
| perft_bulk_d4 | 1.031x | 1.010x | 1.441x |
| perft_full_d3 | 1.124x | 1.020x | 1.502x |
| fen_parse | 1.012x | 0.929x | 1.235x |
| fen_write_native_ep | 0.994x | 0.988x | 1.571x |
| snapshot | 1.009x | 1.006x | 0.946x |
| san_parse_all | 1.002x | 0.993x | 1.831x |
| san_render_native | 1.005x | 0.966x | 2.685x |
| apply_children_full | 1.009x | 0.996x | 1.169x |
| make_unmake_full | 0.996x | 0.921x | 1.545x |
| startpos | 0.999x | 0.999x | 1.304x |
| chess960_materialize | 0.978x | 0.944x | 1.468x |
| chess960_perft_d3 | 1.005x | 0.970x | 1.502x |
| polyglot_key | 1.010x | 0.998x | 26.459x |
| polyglot_recompute | 1.370x | 1.283x | 1.490x |
| repetition_key | 1.012x | 1.001x | 19.670x |
| corpus_count | 1.096x | 1.060x | 1.466x |
| corpus_materialize | 1.003x | 1.003x | 1.223x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.461x |
| replay_trusted_native | 0.982x | 0.953x | 0.611x |
| replay_checked_native | 0.948x | 0.946x | 1.068x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.09x).

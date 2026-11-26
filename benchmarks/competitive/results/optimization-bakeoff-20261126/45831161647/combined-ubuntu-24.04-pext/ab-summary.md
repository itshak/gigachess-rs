| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.984x | 1.397x |
| materialize | 0.996x | 0.984x | 1.507x |
| perft_bulk_d4 | 0.982x | 0.929x | 1.047x |
| perft_full_d3 | 1.018x | 0.952x | 1.462x |
| fen_parse | 1.069x | 0.986x | 1.128x |
| fen_write_native_ep | 1.040x | 1.001x | 1.906x |
| snapshot | 1.000x | 0.997x | 1.154x |
| san_parse_all | 1.004x | 0.992x | 1.878x |
| san_render_native | 1.020x | 1.002x | 2.108x |
| apply_children_full | 1.031x | 1.013x | 1.344x |
| make_unmake_full | 0.964x | 0.915x | 1.133x |
| startpos | 1.111x | 1.111x | 1.110x |
| chess960_materialize | 1.052x | 1.026x | 1.412x |
| chess960_perft_d3 | 0.979x | 0.962x | 1.365x |
| polyglot_key | 0.992x | 0.985x | 39.771x |
| polyglot_recompute | 0.982x | 0.976x | 0.998x |
| repetition_key | 0.994x | 0.984x | 19.488x |
| corpus_count | 1.030x | 1.019x | 0.959x |
| corpus_materialize | 0.992x | 0.988x | 1.055x |
| corpus_uci_sorted | 0.991x | 0.988x | 1.532x |
| replay_trusted_native | 0.988x | 0.965x | 0.906x |
| replay_checked_native | 0.943x | 0.900x | 1.051x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.94x).

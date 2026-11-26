| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.982x | 1.185x |
| materialize | 0.985x | 0.965x | 1.655x |
| perft_bulk_d4 | 0.992x | 0.974x | 1.152x |
| perft_full_d3 | 0.932x | 0.904x | 1.406x |
| fen_parse | 0.944x | 0.887x | 1.103x |
| fen_write_native_ep | 1.007x | 0.974x | 1.768x |
| snapshot | 1.127x | 1.122x | 1.116x |
| san_parse_all | 0.950x | 0.909x | 1.778x |
| san_render_native | 0.934x | 0.911x | 2.048x |
| apply_children_full | 0.919x | 0.908x | 1.112x |
| make_unmake_full | 0.933x | 0.911x | 1.071x |
| startpos | 1.169x | 1.169x | 1.499x |
| chess960_materialize | 0.990x | 0.978x | 1.391x |
| chess960_perft_d3 | 0.967x | 0.956x | 1.280x |
| polyglot_key | 1.017x | 1.006x | 43.956x |
| polyglot_recompute | 1.015x | 1.012x | 1.070x |
| repetition_key | 1.026x | 0.965x | 24.129x |
| corpus_count | 1.005x | 0.999x | 1.276x |
| corpus_materialize | 1.001x | 0.996x | 1.164x |
| corpus_uci_sorted | 1.004x | 1.003x | 1.555x |
| replay_trusted_native | 0.894x | 0.875x | 0.726x |
| replay_checked_native | 0.931x | 0.928x | 1.050x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.89x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.973x | 1.326x |
| materialize | 1.002x | 0.998x | 1.237x |
| perft_bulk_d4 | 0.929x | 0.906x | 1.258x |
| perft_full_d3 | 0.918x | 0.900x | 0.980x |
| fen_parse | 0.972x | 0.952x | 1.210x |
| fen_write_native_ep | 0.989x | 0.978x | 1.604x |
| snapshot | 1.250x | 1.239x | 1.283x |
| san_parse_all | 0.996x | 0.965x | 1.853x |
| san_render_native | 0.754x | 0.695x | 1.914x |
| apply_children_full | 0.790x | 0.723x | 0.835x |
| make_unmake_full | 0.778x | 0.711x | 1.118x |
| startpos | 0.929x | 0.929x | 1.158x |
| chess960_materialize | 0.990x | 0.982x | 1.308x |
| chess960_perft_d3 | 0.936x | 0.928x | 1.242x |
| polyglot_key | 1.106x | 0.929x | 28.453x |
| polyglot_recompute | 0.980x | 0.977x | 1.049x |
| repetition_key | 1.223x | 1.110x | 23.577x |
| corpus_count | 1.000x | 0.998x | 1.444x |
| corpus_materialize | 1.006x | 1.003x | 1.207x |
| corpus_uci_sorted | 1.003x | 0.998x | 1.463x |
| replay_trusted_native | 0.803x | 0.795x | 0.406x |
| replay_checked_native | 0.804x | 0.800x | 0.796x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.89x).

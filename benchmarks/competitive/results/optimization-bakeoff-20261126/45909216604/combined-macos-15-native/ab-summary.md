| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.098x | 0.869x | 1.656x |
| materialize | 1.075x | 0.918x | 1.049x |
| perft_bulk_d4 | 1.133x | 1.001x | 1.365x |
| perft_full_d3 | 0.968x | 0.825x | 1.362x |
| fen_parse | 1.080x | 0.967x | 1.234x |
| fen_write_native_ep | 1.118x | 0.897x | 1.735x |
| snapshot | 1.004x | 0.812x | 1.010x |
| san_parse_all | 0.942x | 0.805x | 1.743x |
| san_render_native | 0.969x | 0.706x | 2.795x |
| apply_children_full | 0.976x | 0.717x | 1.110x |
| make_unmake_full | 1.126x | 0.961x | 1.257x |
| startpos | 1.632x | 1.632x | 1.255x |
| chess960_materialize | 0.908x | 0.880x | 1.411x |
| chess960_perft_d3 | 1.010x | 0.878x | 1.675x |
| polyglot_key | 0.851x | 0.768x | 40.994x |
| polyglot_recompute | 0.872x | 0.691x | 0.980x |
| repetition_key | 1.156x | 1.097x | 20.130x |
| corpus_count | 1.533x | 1.276x | 1.244x |
| corpus_materialize | 0.873x | 0.808x | 1.226x |
| corpus_uci_sorted | 0.798x | 0.735x | 1.355x |
| replay_trusted_native | 0.831x | 0.817x | 0.593x |
| replay_checked_native | 0.904x | 0.779x | 1.345x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.17x).

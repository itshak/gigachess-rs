| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.047x | 0.991x | 1.152x |
| materialize | 1.168x | 1.085x | 1.629x |
| perft_bulk_d4 | 1.205x | 1.132x | 1.147x |
| perft_full_d3 | 1.518x | 1.421x | 0.954x |
| fen_parse | 1.191x | 1.146x | 1.210x |
| fen_write_native_ep | 0.954x | 0.908x | 1.216x |
| snapshot | 1.792x | 1.787x | 1.001x |
| san_parse_all | 2.146x | 2.037x | 1.660x |
| san_render_native | 2.542x | 2.264x | 2.125x |
| apply_children_full | 2.323x | 2.111x | 0.986x |
| make_unmake_full | 1.294x | 1.218x | 1.030x |
| startpos | 1.713x | 1.713x | 1.354x |
| chess960_materialize | 1.167x | 1.075x | 1.355x |
| chess960_perft_d3 | 1.281x | 1.189x | 1.277x |
| polyglot_key | 0.958x | 0.853x | 39.888x |
| polyglot_recompute | 0.983x | 0.977x | 1.004x |
| repetition_key | 1.488x | 1.462x | 22.300x |
| corpus_count | 1.043x | 1.039x | 1.243x |
| corpus_materialize | 1.019x | 1.015x | 1.168x |
| corpus_uci_sorted | 1.523x | 1.508x | 0.969x |
| replay_trusted_native | 1.228x | 1.164x | 0.631x |
| replay_checked_native | 1.219x | 1.125x | 0.965x |

Cold first legal move list: 134.442 ms → 0.009 ms (15261.90x).

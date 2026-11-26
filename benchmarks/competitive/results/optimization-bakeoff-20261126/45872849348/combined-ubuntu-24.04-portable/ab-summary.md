| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.993x | 1.323x |
| materialize | 0.999x | 0.992x | 1.351x |
| perft_bulk_d4 | 0.943x | 0.918x | 1.226x |
| perft_full_d3 | 0.985x | 0.972x | 1.230x |
| fen_parse | 1.118x | 1.086x | 1.210x |
| fen_write_native_ep | 1.043x | 1.001x | 1.689x |
| snapshot | 1.000x | 0.997x | 1.031x |
| san_parse_all | 0.993x | 0.977x | 2.042x |
| san_render_native | 1.008x | 0.957x | 2.223x |
| apply_children_full | 0.904x | 0.839x | 1.273x |
| make_unmake_full | 0.917x | 0.876x | 1.321x |
| startpos | 1.000x | 1.000x | 1.325x |
| chess960_materialize | 0.994x | 0.991x | 1.466x |
| chess960_perft_d3 | 0.965x | 0.944x | 1.498x |
| polyglot_key | 1.077x | 0.999x | 28.149x |
| polyglot_recompute | 1.016x | 1.013x | 1.052x |
| repetition_key | 1.110x | 1.062x | 21.300x |
| corpus_count | 0.999x | 0.997x | 1.281x |
| corpus_materialize | 0.997x | 0.991x | 1.184x |
| corpus_uci_sorted | 0.999x | 0.995x | 1.753x |
| replay_trusted_native | 0.989x | 0.890x | 0.906x |
| replay_checked_native | 0.995x | 0.991x | 1.135x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.98x).

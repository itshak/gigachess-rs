| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.989x | 1.548x |
| materialize | 1.004x | 0.996x | 1.323x |
| perft_bulk_d4 | 0.993x | 0.982x | 1.381x |
| perft_full_d3 | 1.004x | 1.002x | 1.332x |
| fen_parse | 0.998x | 0.993x | 1.222x |
| fen_write_native_ep | 0.996x | 0.994x | 1.566x |
| snapshot | 0.992x | 0.989x | 0.943x |
| san_parse_all | 0.999x | 0.989x | 1.817x |
| san_render_native | 1.003x | 0.991x | 2.678x |
| apply_children_full | 0.943x | 0.918x | 1.095x |
| make_unmake_full | 0.961x | 0.942x | 1.496x |
| startpos | 1.000x | 1.000x | 1.304x |
| chess960_materialize | 1.000x | 0.997x | 1.487x |
| chess960_perft_d3 | 1.004x | 0.997x | 1.503x |
| polyglot_key | 0.989x | 0.983x | 25.667x |
| polyglot_recompute | 1.000x | 0.997x | 1.093x |
| repetition_key | 0.997x | 0.996x | 19.240x |
| corpus_count | 0.998x | 0.998x | 1.347x |
| corpus_materialize | 0.992x | 0.990x | 1.228x |
| corpus_uci_sorted | 0.998x | 0.991x | 1.451x |
| replay_trusted_native | 0.915x | 0.893x | 0.579x |
| replay_checked_native | 0.960x | 0.959x | 1.082x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.06x).

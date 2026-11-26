| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.976x | 1.276x |
| materialize | 1.037x | 1.023x | 1.349x |
| perft_bulk_d4 | 1.007x | 0.981x | 1.264x |
| perft_full_d3 | 1.006x | 0.997x | 1.301x |
| fen_parse | 0.877x | 0.854x | 1.026x |
| fen_write_native_ep | 0.994x | 0.980x | 1.750x |
| snapshot | 0.999x | 0.994x | 0.976x |
| san_parse_all | 0.983x | 0.965x | 2.133x |
| san_render_native | 1.020x | 0.991x | 2.302x |
| apply_children_full | 1.122x | 1.070x | 1.409x |
| make_unmake_full | 0.993x | 0.948x | 1.394x |
| startpos | 1.001x | 1.001x | 1.338x |
| chess960_materialize | 1.087x | 1.075x | 1.675x |
| chess960_perft_d3 | 0.992x | 0.981x | 1.568x |
| polyglot_key | 0.997x | 0.994x | 21.737x |
| polyglot_recompute | 1.007x | 0.996x | 0.975x |
| repetition_key | 1.007x | 0.995x | 20.165x |
| corpus_count | 1.000x | 0.999x | 1.236x |
| corpus_materialize | 1.002x | 1.001x | 1.382x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.594x |
| replay_trusted_native | 1.189x | 1.143x | 1.131x |
| replay_checked_native | 1.087x | 1.080x | 1.158x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.05x).

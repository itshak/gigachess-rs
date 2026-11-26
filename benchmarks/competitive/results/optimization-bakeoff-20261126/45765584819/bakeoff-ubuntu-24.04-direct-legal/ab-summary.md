| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 1.000x | 1.188x |
| materialize | 1.011x | 1.003x | 1.392x |
| perft_bulk_d4 | 0.994x | 0.985x | 0.870x |
| perft_full_d3 | 0.994x | 0.986x | 0.638x |
| fen_parse | 1.001x | 0.994x | 1.049x |
| fen_write_native_ep | 1.012x | 0.999x | 1.357x |
| snapshot | 0.568x | 0.563x | 0.858x |
| san_parse_all | 1.691x | 1.649x | 1.553x |
| san_render_native | 1.007x | 1.003x | 1.068x |
| apply_children_full | 0.812x | 0.797x | 0.679x |
| make_unmake_full | 1.000x | 0.983x | 0.757x |
| startpos | 0.888x | 0.888x | 0.888x |
| chess960_materialize | 1.052x | 1.029x | 1.200x |
| chess960_perft_d3 | 0.997x | 0.993x | 0.954x |
| polyglot_key | 0.982x | 0.967x | 35.150x |
| polyglot_recompute | 1.023x | 1.022x | 1.007x |
| repetition_key | 1.068x | 0.989x | 16.926x |
| corpus_count | 0.997x | 0.993x | 1.158x |
| corpus_materialize | 0.996x | 0.993x | 1.074x |
| corpus_uci_sorted | 0.983x | 0.981x | 0.606x |
| replay_trusted_native | 0.960x | 0.943x | 0.500x |
| replay_checked_native | 1.000x | 0.982x | 0.800x |

Cold first legal move list: 142.463 ms → 140.214 ms (1.02x).

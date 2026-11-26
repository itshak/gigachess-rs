| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.378x |
| materialize | 0.995x | 0.984x | 1.236x |
| perft_bulk_d4 | 1.001x | 0.995x | 1.241x |
| perft_full_d3 | 0.927x | 0.866x | 0.993x |
| fen_parse | 0.988x | 0.976x | 1.227x |
| fen_write_native_ep | 0.994x | 0.980x | 1.586x |
| snapshot | 0.993x | 0.991x | 0.960x |
| san_parse_all | 0.991x | 0.982x | 1.804x |
| san_render_native | 0.955x | 0.919x | 2.422x |
| apply_children_full | 0.944x | 0.893x | 1.111x |
| make_unmake_full | 0.900x | 0.858x | 1.333x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 0.914x | 0.840x | 1.266x |
| chess960_perft_d3 | 0.942x | 0.887x | 1.264x |
| polyglot_key | 0.994x | 0.987x | 25.161x |
| polyglot_recompute | 1.002x | 1.001x | 1.073x |
| repetition_key | 0.996x | 0.995x | 18.306x |
| corpus_count | 0.993x | 0.990x | 1.213x |
| corpus_materialize | 1.001x | 1.000x | 1.174x |
| corpus_uci_sorted | 0.997x | 0.996x | 1.449x |
| replay_trusted_native | 0.892x | 0.832x | 0.498x |
| replay_checked_native | 0.925x | 0.895x | 0.943x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.39x).

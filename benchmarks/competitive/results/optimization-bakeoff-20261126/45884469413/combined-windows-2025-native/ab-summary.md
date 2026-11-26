| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.033x | 0.978x | 1.500x |
| materialize | 1.013x | 0.989x | 1.300x |
| perft_bulk_d4 | 1.040x | 0.987x | 1.383x |
| perft_full_d3 | 1.038x | 0.997x | 1.336x |
| fen_parse | 1.077x | 0.993x | 1.277x |
| fen_write_native_ep | 1.034x | 0.989x | 1.679x |
| snapshot | 1.011x | 0.994x | 0.999x |
| san_parse_all | 1.056x | 1.005x | 2.139x |
| san_render_native | 1.042x | 0.995x | 3.828x |
| apply_children_full | 1.025x | 0.999x | 1.396x |
| make_unmake_full | 1.049x | 1.008x | 1.368x |
| startpos | 1.003x | 1.003x | 1.246x |
| chess960_materialize | 1.226x | 1.162x | 1.355x |
| chess960_perft_d3 | 1.069x | 1.013x | 1.551x |
| polyglot_key | 1.044x | 1.000x | 27.312x |
| polyglot_recompute | 1.112x | 0.998x | 1.003x |
| repetition_key | 1.070x | 0.983x | 20.460x |
| corpus_count | 1.160x | 1.141x | 1.406x |
| corpus_materialize | 1.128x | 1.013x | 1.202x |
| corpus_uci_sorted | 1.087x | 1.003x | 1.453x |
| replay_trusted_native | 1.048x | 1.003x | 0.847x |
| replay_checked_native | 1.005x | 0.986x | 1.100x |

Cold first legal move list: 0.016 ms -> 0.015 ms (1.10x).

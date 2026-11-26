| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 1.010x | 1.546x |
| materialize | 0.961x | 0.929x | 1.567x |
| perft_bulk_d4 | 1.004x | 0.992x | 1.175x |
| perft_full_d3 | 1.249x | 1.060x | 1.710x |
| fen_parse | 0.973x | 0.943x | 1.125x |
| fen_write_native_ep | 1.000x | 0.985x | 1.924x |
| snapshot | 0.998x | 0.990x | 1.151x |
| san_parse_all | 0.900x | 0.884x | 1.697x |
| san_render_native | 1.000x | 0.997x | 2.112x |
| apply_children_full | 0.999x | 0.995x | 1.332x |
| make_unmake_full | 0.992x | 0.977x | 1.190x |
| startpos | 0.996x | 0.996x | 1.118x |
| chess960_materialize | 1.001x | 0.986x | 1.549x |
| chess960_perft_d3 | 0.983x | 0.978x | 1.487x |
| polyglot_key | 1.008x | 0.993x | 40.878x |
| polyglot_recompute | 1.096x | 0.882x | 1.126x |
| repetition_key | 1.238x | 1.080x | 24.577x |
| corpus_count | 1.026x | 1.021x | 1.105x |
| corpus_materialize | 1.005x | 1.005x | 1.045x |
| corpus_uci_sorted | 0.991x | 0.990x | 1.538x |
| replay_trusted_native | 0.980x | 0.956x | 0.864x |
| replay_checked_native | 1.013x | 1.007x | 1.221x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).

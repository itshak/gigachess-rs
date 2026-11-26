| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.124x | 0.966x | 1.547x |
| materialize | 1.052x | 0.981x | 1.331x |
| perft_bulk_d4 | 1.119x | 1.001x | 1.395x |
| perft_full_d3 | 1.011x | 0.993x | 1.062x |
| fen_parse | 0.998x | 0.989x | 1.219x |
| fen_write_native_ep | 0.999x | 0.997x | 1.591x |
| snapshot | 1.014x | 1.008x | 0.943x |
| san_parse_all | 0.997x | 0.988x | 1.843x |
| san_render_native | 0.998x | 0.989x | 2.511x |
| apply_children_full | 0.999x | 0.985x | 1.150x |
| make_unmake_full | 1.005x | 0.977x | 1.542x |
| startpos | 0.999x | 0.999x | 1.296x |
| chess960_materialize | 1.080x | 1.041x | 1.500x |
| chess960_perft_d3 | 1.117x | 1.083x | 1.501x |
| polyglot_key | 0.995x | 0.993x | 25.322x |
| polyglot_recompute | 0.998x | 0.996x | 1.082x |
| repetition_key | 1.011x | 1.001x | 19.441x |
| corpus_count | 1.148x | 1.098x | 1.381x |
| corpus_materialize | 1.047x | 1.034x | 1.234x |
| corpus_uci_sorted | 1.008x | 1.004x | 1.458x |
| replay_trusted_native | 1.020x | 1.006x | 0.628x |
| replay_checked_native | 0.994x | 0.987x | 1.048x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).

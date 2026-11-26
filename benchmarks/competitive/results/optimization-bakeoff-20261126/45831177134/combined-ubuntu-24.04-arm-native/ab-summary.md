| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.378x |
| materialize | 0.998x | 0.982x | 1.258x |
| perft_bulk_d4 | 0.999x | 0.989x | 1.243x |
| perft_full_d3 | 0.999x | 0.996x | 1.049x |
| fen_parse | 1.009x | 1.004x | 1.225x |
| fen_write_native_ep | 0.990x | 0.978x | 1.565x |
| snapshot | 1.001x | 0.998x | 0.946x |
| san_parse_all | 1.000x | 0.997x | 1.865x |
| san_render_native | 1.006x | 1.000x | 2.531x |
| apply_children_full | 0.995x | 0.986x | 1.160x |
| make_unmake_full | 1.031x | 0.993x | 1.581x |
| startpos | 0.999x | 0.999x | 1.302x |
| chess960_materialize | 1.002x | 1.001x | 1.390x |
| chess960_perft_d3 | 1.000x | 0.996x | 1.348x |
| polyglot_key | 0.991x | 0.987x | 25.409x |
| polyglot_recompute | 1.002x | 1.002x | 1.093x |
| repetition_key | 1.001x | 1.000x | 19.213x |
| corpus_count | 1.004x | 0.999x | 1.200x |
| corpus_materialize | 1.002x | 0.998x | 1.192x |
| corpus_uci_sorted | 1.005x | 1.003x | 1.447x |
| replay_trusted_native | 1.017x | 1.013x | 0.628x |
| replay_checked_native | 0.992x | 0.985x | 1.042x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.07x).

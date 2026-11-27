| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.986x | 1.565x |
| materialize | 1.010x | 1.001x | 1.332x |
| perft_bulk_d4 | 1.045x | 1.023x | 1.474x |
| perft_full_d3 | 1.135x | 1.005x | 1.515x |
| fen_parse | 0.983x | 0.963x | 1.190x |
| fen_write_native_ep | 0.991x | 0.980x | 1.569x |
| snapshot | 1.006x | 1.002x | 0.936x |
| san_parse_all | 0.999x | 0.997x | 1.824x |
| san_render_native | 0.999x | 0.992x | 2.697x |
| apply_children_full | 0.996x | 0.968x | 1.157x |
| make_unmake_full | 1.019x | 1.001x | 1.575x |
| startpos | 1.001x | 1.001x | 1.275x |
| chess960_materialize | 1.002x | 1.002x | 1.500x |
| chess960_perft_d3 | 1.015x | 1.002x | 1.516x |
| polyglot_key | 1.010x | 0.997x | 25.008x |
| polyglot_recompute | 0.868x | 0.856x | 0.939x |
| repetition_key | 1.004x | 1.001x | 18.844x |
| corpus_count | 1.099x | 1.069x | 1.514x |
| corpus_materialize | 1.007x | 1.007x | 1.235x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.463x |
| replay_trusted_native | 1.014x | 1.001x | 0.630x |
| replay_checked_native | 1.013x | 1.009x | 1.141x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).

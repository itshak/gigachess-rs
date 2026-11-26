| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.551x |
| materialize | 1.001x | 0.999x | 1.319x |
| perft_bulk_d4 | 1.000x | 0.995x | 1.390x |
| perft_full_d3 | 1.009x | 1.004x | 0.838x |
| fen_parse | 1.004x | 0.996x | 1.213x |
| fen_write_native_ep | 0.999x | 0.997x | 1.582x |
| snapshot | 1.001x | 0.999x | 0.948x |
| san_parse_all | 1.001x | 1.000x | 1.839x |
| san_render_native | 1.003x | 0.996x | 2.561x |
| apply_children_full | 0.995x | 0.972x | 1.151x |
| make_unmake_full | 1.022x | 1.004x | 1.578x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 1.002x | 1.000x | 1.498x |
| chess960_perft_d3 | 0.996x | 0.993x | 1.500x |
| polyglot_key | 1.007x | 0.997x | 25.886x |
| polyglot_recompute | 0.996x | 0.990x | 1.088x |
| repetition_key | 1.002x | 1.000x | 19.444x |
| corpus_count | 1.005x | 1.002x | 1.370x |
| corpus_materialize | 1.005x | 1.005x | 1.232x |
| corpus_uci_sorted | 1.000x | 0.997x | 1.455x |
| replay_trusted_native | 1.019x | 1.015x | 0.619x |
| replay_checked_native | 1.097x | 1.074x | 1.146x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.08x).

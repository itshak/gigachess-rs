| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.976x | 1.555x |
| materialize | 1.037x | 1.008x | 1.511x |
| perft_bulk_d4 | 1.011x | 1.000x | 1.198x |
| perft_full_d3 | 1.006x | 1.000x | 1.400x |
| fen_parse | 0.934x | 0.831x | 1.078x |
| fen_write_native_ep | 1.020x | 0.975x | 2.095x |
| snapshot | 1.001x | 0.995x | 1.154x |
| san_parse_all | 1.030x | 0.994x | 1.888x |
| san_render_native | 1.013x | 1.006x | 2.098x |
| apply_children_full | 1.009x | 1.001x | 1.348x |
| make_unmake_full | 0.941x | 0.886x | 1.132x |
| startpos | 1.116x | 1.116x | 1.115x |
| chess960_materialize | 1.082x | 1.077x | 1.488x |
| chess960_perft_d3 | 0.996x | 0.992x | 1.473x |
| polyglot_key | 1.094x | 1.000x | 41.551x |
| polyglot_recompute | 0.959x | 0.948x | 0.998x |
| repetition_key | 0.888x | 0.703x | 20.878x |
| corpus_count | 0.997x | 0.993x | 1.120x |
| corpus_materialize | 1.010x | 1.005x | 1.093x |
| corpus_uci_sorted | 1.023x | 1.019x | 1.559x |
| replay_trusted_native | 0.968x | 0.910x | 0.856x |
| replay_checked_native | 1.001x | 0.988x | 1.128x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.05x).

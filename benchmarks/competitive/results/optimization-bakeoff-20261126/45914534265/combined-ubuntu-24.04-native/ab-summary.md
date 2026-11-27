| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.507x |
| materialize | 1.001x | 0.985x | 1.177x |
| perft_bulk_d4 | 0.996x | 0.986x | 1.339x |
| perft_full_d3 | 0.998x | 0.989x | 1.205x |
| fen_parse | 1.010x | 0.985x | 1.186x |
| fen_write_native_ep | 0.998x | 0.990x | 1.839x |
| snapshot | 1.907x | 1.886x | 1.248x |
| san_parse_all | 1.004x | 0.996x | 2.204x |
| san_render_native | 0.987x | 0.943x | 2.072x |
| apply_children_full | 1.008x | 0.979x | 1.797x |
| make_unmake_full | 0.912x | 0.875x | 1.105x |
| startpos | 1.000x | 1.000x | 1.197x |
| chess960_materialize | 0.997x | 0.995x | 1.556x |
| chess960_perft_d3 | 1.001x | 1.000x | 1.603x |
| polyglot_key | 1.004x | 1.000x | 20.552x |
| polyglot_recompute | 0.952x | 0.940x | 0.975x |
| repetition_key | 0.995x | 0.991x | 18.958x |
| corpus_count | 0.999x | 0.997x | 1.299x |
| corpus_materialize | 1.000x | 0.998x | 1.234x |
| corpus_uci_sorted | 0.984x | 0.983x | 1.660x |
| replay_trusted_native | 1.186x | 1.009x | 1.189x |
| replay_checked_native | 1.024x | 1.008x | 1.099x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).

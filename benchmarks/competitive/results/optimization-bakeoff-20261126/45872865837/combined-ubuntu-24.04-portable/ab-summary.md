| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 1.003x | 1.318x |
| materialize | 0.996x | 0.987x | 1.373x |
| perft_bulk_d4 | 0.998x | 0.986x | 1.305x |
| perft_full_d3 | 1.203x | 1.038x | 1.511x |
| fen_parse | 1.002x | 0.980x | 1.094x |
| fen_write_native_ep | 1.078x | 1.012x | 1.685x |
| snapshot | 0.999x | 0.997x | 0.967x |
| san_parse_all | 0.998x | 0.995x | 1.995x |
| san_render_native | 1.020x | 0.982x | 2.289x |
| apply_children_full | 0.884x | 0.837x | 1.253x |
| make_unmake_full | 0.895x | 0.840x | 1.301x |
| startpos | 1.000x | 1.000x | 1.339x |
| chess960_materialize | 0.998x | 0.994x | 1.480x |
| chess960_perft_d3 | 1.001x | 0.997x | 1.563x |
| polyglot_key | 0.994x | 0.992x | 27.495x |
| polyglot_recompute | 0.986x | 0.979x | 1.003x |
| repetition_key | 0.992x | 0.979x | 19.439x |
| corpus_count | 0.996x | 0.988x | 1.281x |
| corpus_materialize | 0.986x | 0.986x | 1.196x |
| corpus_uci_sorted | 0.997x | 0.997x | 1.819x |
| replay_trusted_native | 0.795x | 0.788x | 0.719x |
| replay_checked_native | 0.990x | 0.986x | 1.132x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.00x).

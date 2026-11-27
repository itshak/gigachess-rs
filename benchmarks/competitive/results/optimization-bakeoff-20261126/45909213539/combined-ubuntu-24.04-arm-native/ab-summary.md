| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.548x |
| materialize | 0.998x | 0.995x | 1.316x |
| perft_bulk_d4 | 1.001x | 0.999x | 1.392x |
| perft_full_d3 | 1.000x | 0.999x | 1.330x |
| fen_parse | 1.001x | 0.997x | 1.229x |
| fen_write_native_ep | 0.995x | 0.992x | 1.578x |
| snapshot | 1.000x | 0.996x | 0.958x |
| san_parse_all | 0.999x | 0.990x | 1.852x |
| san_render_native | 1.004x | 0.997x | 2.672x |
| apply_children_full | 1.000x | 0.998x | 1.150x |
| make_unmake_full | 1.003x | 0.986x | 1.564x |
| startpos | 1.002x | 1.002x | 1.333x |
| chess960_materialize | 0.998x | 0.997x | 1.500x |
| chess960_perft_d3 | 0.999x | 0.997x | 1.495x |
| polyglot_key | 0.997x | 0.995x | 25.471x |
| polyglot_recompute | 1.000x | 0.998x | 1.088x |
| repetition_key | 1.017x | 1.001x | 19.325x |
| corpus_count | 1.003x | 0.999x | 1.372x |
| corpus_materialize | 1.002x | 0.997x | 1.236x |
| corpus_uci_sorted | 1.000x | 0.996x | 1.458x |
| replay_trusted_native | 1.007x | 0.999x | 0.627x |
| replay_checked_native | 0.999x | 0.997x | 1.136x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.01x).

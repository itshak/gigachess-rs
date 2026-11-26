| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.548x |
| materialize | 1.000x | 0.997x | 1.319x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.400x |
| perft_full_d3 | 1.000x | 1.000x | 1.328x |
| fen_parse | 0.997x | 0.991x | 1.208x |
| fen_write_native_ep | 1.003x | 0.998x | 1.576x |
| snapshot | 1.000x | 0.997x | 0.949x |
| san_parse_all | 1.003x | 0.997x | 1.838x |
| san_render_native | 0.998x | 0.993x | 2.586x |
| apply_children_full | 1.003x | 0.990x | 1.159x |
| make_unmake_full | 1.074x | 1.035x | 1.638x |
| startpos | 1.000x | 1.000x | 1.306x |
| chess960_materialize | 0.995x | 0.995x | 1.498x |
| chess960_perft_d3 | 1.002x | 0.996x | 1.495x |
| polyglot_key | 0.998x | 0.997x | 25.452x |
| polyglot_recompute | 1.000x | 0.999x | 1.086x |
| repetition_key | 0.993x | 0.985x | 19.241x |
| corpus_count | 1.007x | 0.998x | 1.347x |
| corpus_materialize | 1.004x | 0.998x | 1.208x |
| corpus_uci_sorted | 1.003x | 1.001x | 1.459x |
| replay_trusted_native | 1.096x | 1.052x | 0.686x |
| replay_checked_native | 1.014x | 1.013x | 1.065x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.21x).

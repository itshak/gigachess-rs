| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.991x | 1.562x |
| materialize | 0.997x | 0.991x | 1.489x |
| perft_bulk_d4 | 1.013x | 0.986x | 1.202x |
| perft_full_d3 | 0.997x | 0.994x | 1.350x |
| fen_parse | 0.999x | 0.970x | 1.047x |
| fen_write_native_ep | 0.958x | 0.909x | 1.836x |
| snapshot | 0.997x | 0.986x | 1.149x |
| san_parse_all | 1.087x | 1.071x | 1.871x |
| san_render_native | 0.998x | 0.996x | 2.102x |
| apply_children_full | 0.994x | 0.970x | 1.293x |
| make_unmake_full | 0.980x | 0.967x | 1.152x |
| startpos | 1.014x | 1.014x | 1.125x |
| chess960_materialize | 1.012x | 1.010x | 1.413x |
| chess960_perft_d3 | 0.998x | 0.995x | 1.465x |
| polyglot_key | 1.003x | 0.999x | 39.766x |
| polyglot_recompute | 1.050x | 1.043x | 1.024x |
| repetition_key | 0.815x | 0.699x | 19.265x |
| corpus_count | 0.997x | 0.995x | 1.050x |
| corpus_materialize | 1.000x | 0.998x | 1.088x |
| corpus_uci_sorted | 1.003x | 1.002x | 1.594x |
| replay_trusted_native | 0.967x | 0.931x | 0.876x |
| replay_checked_native | 1.021x | 1.019x | 1.081x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.15x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.990x | 1.278x |
| materialize | 0.910x | 0.888x | 1.686x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.181x |
| perft_full_d3 | 1.103x | 1.012x | 1.471x |
| fen_parse | 1.000x | 0.987x | 1.164x |
| fen_write_native_ep | 0.982x | 0.958x | 1.929x |
| snapshot | 1.000x | 1.000x | 1.000x |
| san_parse_all | 1.007x | 1.003x | 1.885x |
| san_render_native | 1.019x | 1.010x | 2.241x |
| apply_children_full | 1.002x | 0.996x | 1.319x |
| make_unmake_full | 0.998x | 0.982x | 1.233x |
| startpos | 1.001x | 1.001x | 1.290x |
| chess960_materialize | 0.997x | 0.995x | 1.551x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.504x |
| polyglot_key | 0.931x | 0.868x | 38.367x |
| polyglot_recompute | 0.990x | 0.984x | 0.959x |
| repetition_key | 1.074x | 0.854x | 21.869x |
| corpus_count | 0.987x | 0.981x | 1.168x |
| corpus_materialize | 1.000x | 0.995x | 1.129x |
| corpus_uci_sorted | 1.007x | 1.006x | 1.577x |
| replay_trusted_native | 0.981x | 0.974x | 0.878x |
| replay_checked_native | 1.007x | 1.005x | 1.067x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.15x).

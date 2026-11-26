| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.995x | 1.086x |
| materialize | 1.004x | 1.002x | 1.175x |
| perft_bulk_d4 | 0.998x | 0.994x | 0.899x |
| perft_full_d3 | 1.000x | 0.998x | 0.610x |
| fen_parse | 0.988x | 0.978x | 1.195x |
| fen_write_native_ep | 1.007x | 0.994x | 1.062x |
| snapshot | 1.001x | 0.997x | 0.803x |
| san_parse_all | 1.010x | 1.004x | 0.920x |
| san_render_native | 1.000x | 0.992x | 1.146x |
| apply_children_full | 1.012x | 1.004x | 0.656x |
| make_unmake_full | 1.006x | 1.000x | 0.891x |
| startpos | 0.972x | 0.972x | 0.825x |
| chess960_materialize | 1.003x | 1.003x | 1.111x |
| chess960_perft_d3 | 1.004x | 1.001x | 0.844x |
| polyglot_key | 1.009x | 1.009x | 22.857x |
| polyglot_recompute | 0.994x | 0.991x | 1.127x |
| repetition_key | 1.014x | 1.002x | 13.852x |
| corpus_count | 1.021x | 1.010x | 1.235x |
| corpus_materialize | 1.008x | 1.005x | 1.131x |
| corpus_uci_sorted | 1.308x | 1.290x | 0.766x |
| replay_trusted_native | 1.004x | 1.001x | 0.323x |
| replay_checked_native | 1.005x | 1.001x | 0.661x |

Cold first legal move list: 153.928 ms → 154.033 ms (1.00x).

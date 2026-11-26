| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 1.000x | 1.361x |
| materialize | 0.996x | 0.987x | 1.078x |
| perft_bulk_d4 | 0.995x | 0.985x | 1.331x |
| perft_full_d3 | 0.946x | 0.906x | 1.024x |
| fen_parse | 1.009x | 0.997x | 1.241x |
| fen_write_native_ep | 1.006x | 0.997x | 1.594x |
| snapshot | 0.717x | 0.706x | 0.782x |
| san_parse_all | 1.012x | 0.992x | 2.024x |
| san_render_native | 0.858x | 0.807x | 2.998x |
| apply_children_full | 0.998x | 0.980x | 1.036x |
| make_unmake_full | 0.996x | 0.960x | 1.238x |
| startpos | 1.098x | 1.098x | 1.184x |
| chess960_materialize | 1.011x | 1.004x | 1.221x |
| chess960_perft_d3 | 1.012x | 1.003x | 1.427x |
| polyglot_key | 0.994x | 0.992x | 20.277x |
| polyglot_recompute | 0.977x | 0.968x | 0.962x |
| repetition_key | 0.996x | 0.995x | 16.724x |
| corpus_count | 1.013x | 1.011x | 1.308x |
| corpus_materialize | 1.005x | 1.004x | 1.155x |
| corpus_uci_sorted | 0.997x | 0.997x | 1.499x |
| replay_trusted_native | 1.162x | 1.110x | 0.759x |
| replay_checked_native | 1.037x | 1.012x | 0.974x |

Cold first legal move list: 0.023 ms -> 0.022 ms (1.05x).

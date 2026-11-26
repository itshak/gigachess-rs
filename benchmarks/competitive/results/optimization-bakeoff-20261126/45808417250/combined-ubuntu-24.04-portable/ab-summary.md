| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.994x | 1.167x |
| materialize | 1.001x | 0.987x | 1.694x |
| perft_bulk_d4 | 1.009x | 1.000x | 1.166x |
| perft_full_d3 | 1.004x | 0.994x | 1.510x |
| fen_parse | 1.035x | 0.983x | 1.153x |
| fen_write_native_ep | 1.003x | 0.997x | 2.040x |
| snapshot | 0.803x | 0.797x | 0.998x |
| san_parse_all | 1.010x | 0.983x | 1.880x |
| san_render_native | 0.993x | 0.982x | 2.174x |
| apply_children_full | 0.969x | 0.951x | 1.185x |
| make_unmake_full | 0.960x | 0.926x | 1.113x |
| startpos | 0.990x | 0.990x | 1.279x |
| chess960_materialize | 0.931x | 0.927x | 1.325x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.326x |
| polyglot_key | 1.010x | 1.002x | 41.131x |
| polyglot_recompute | 0.982x | 0.972x | 0.972x |
| repetition_key | 0.684x | 0.600x | 14.350x |
| corpus_count | 0.999x | 0.992x | 1.268x |
| corpus_materialize | 0.997x | 0.994x | 1.156x |
| corpus_uci_sorted | 1.008x | 1.002x | 1.573x |
| replay_trusted_native | 0.948x | 0.923x | 0.741x |
| replay_checked_native | 0.977x | 0.967x | 1.064x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.87x).

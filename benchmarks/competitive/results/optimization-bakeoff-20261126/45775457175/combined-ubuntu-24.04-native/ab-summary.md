| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.375x |
| materialize | 0.960x | 0.936x | 1.434x |
| perft_bulk_d4 | 1.004x | 0.997x | 1.172x |
| perft_full_d3 | 0.961x | 0.952x | 0.847x |
| fen_parse | 1.078x | 1.049x | 1.163x |
| fen_write_native_ep | 1.037x | 1.015x | 1.306x |
| snapshot | 1.662x | 1.656x | 1.331x |
| san_parse_all | 1.064x | 1.035x | 1.755x |
| san_render_native | 1.014x | 1.004x | 2.101x |
| apply_children_full | 0.930x | 0.905x | 0.976x |
| make_unmake_full | 1.003x | 0.972x | 0.940x |
| startpos | 1.190x | 1.190x | 0.992x |
| chess960_materialize | 1.076x | 1.071x | 1.374x |
| chess960_perft_d3 | 1.007x | 1.001x | 1.322x |
| polyglot_key | 0.997x | 0.997x | 38.785x |
| polyglot_recompute | 1.039x | 1.035x | 1.016x |
| repetition_key | 0.866x | 0.750x | 18.900x |
| corpus_count | 0.999x | 0.999x | 1.295x |
| corpus_materialize | 0.998x | 0.996x | 1.078x |
| corpus_uci_sorted | 0.999x | 0.999x | 0.960x |
| replay_trusted_native | 0.983x | 0.971x | 0.595x |
| replay_checked_native | 1.007x | 0.965x | 0.982x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.94x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.945x | 0.896x | 1.326x |
| materialize | 1.017x | 0.957x | 1.212x |
| perft_bulk_d4 | 0.890x | 0.825x | 1.249x |
| perft_full_d3 | 0.946x | 0.856x | 1.130x |
| fen_parse | 1.170x | 1.151x | 1.192x |
| fen_write_native_ep | 1.032x | 0.983x | 1.767x |
| snapshot | 1.760x | 1.749x | 1.250x |
| san_parse_all | 1.004x | 0.977x | 2.071x |
| san_render_native | 0.958x | 0.911x | 2.193x |
| apply_children_full | 1.078x | 1.033x | 1.238x |
| make_unmake_full | 0.989x | 0.973x | 1.263x |
| startpos | 0.998x | 0.998x | 1.249x |
| chess960_materialize | 1.037x | 0.971x | 1.293x |
| chess960_perft_d3 | 1.024x | 0.990x | 1.375x |
| polyglot_key | 0.997x | 0.988x | 29.480x |
| polyglot_recompute | 1.019x | 0.988x | 1.051x |
| repetition_key | 0.869x | 0.851x | 19.710x |
| corpus_count | 0.854x | 0.771x | 1.279x |
| corpus_materialize | 0.948x | 0.918x | 1.106x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.719x |
| replay_trusted_native | 0.994x | 0.984x | 0.767x |
| replay_checked_native | 0.996x | 0.969x | 1.008x |

Cold first legal move list: 0.014 ms -> 0.009 ms (1.47x).

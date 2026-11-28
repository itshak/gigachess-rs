| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.997x | 1.512x |
| materialize | 0.976x | 0.931x | 1.199x |
| perft_bulk_d4 | 1.066x | 1.025x | 1.344x |
| perft_full_d3 | 1.206x | 0.865x | 1.499x |
| fen_parse | 0.957x | 0.922x | 1.224x |
| fen_write_native_ep | 0.992x | 0.976x | 1.548x |
| snapshot | 1.002x | 0.993x | 1.495x |
| san_parse_all | 1.000x | 0.991x | 1.910x |
| san_render_native | 1.010x | 0.969x | 2.290x |
| apply_children_full | 0.998x | 0.959x | 1.364x |
| make_unmake_full | 0.981x | 0.939x | 1.168x |
| startpos | 0.997x | 0.997x | 1.250x |
| chess960_materialize | 0.986x | 0.980x | 1.517x |
| chess960_perft_d3 | 0.994x | 0.967x | 1.470x |
| polyglot_key | 0.995x | 0.994x | 34.885x |
| polyglot_recompute | 1.596x | 1.340x | 1.560x |
| repetition_key | 0.996x | 0.987x | 27.185x |
| corpus_count | 1.076x | 1.045x | 1.246x |
| corpus_materialize | 1.067x | 1.063x | 1.129x |
| corpus_uci_sorted | 0.973x | 0.968x | 1.641x |
| replay_trusted_native | 1.034x | 1.018x | 0.767x |
| replay_checked_native | 1.013x | 1.008x | 1.177x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.95x).

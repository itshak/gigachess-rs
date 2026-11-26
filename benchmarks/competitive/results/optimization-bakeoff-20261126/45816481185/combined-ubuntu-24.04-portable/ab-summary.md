| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.978x | 1.191x |
| materialize | 1.013x | 0.973x | 1.259x |
| perft_bulk_d4 | 1.014x | 0.999x | 1.181x |
| perft_full_d3 | 1.007x | 0.981x | 1.324x |
| fen_parse | 0.945x | 0.918x | 1.292x |
| fen_write_native_ep | 0.991x | 0.959x | 1.691x |
| snapshot | 0.997x | 0.991x | 0.971x |
| san_parse_all | 0.987x | 0.974x | 2.001x |
| san_render_native | 0.995x | 0.968x | 2.320x |
| apply_children_full | 1.003x | 0.997x | 1.279x |
| make_unmake_full | 0.993x | 0.980x | 1.360x |
| startpos | 1.000x | 1.000x | 1.336x |
| chess960_materialize | 1.002x | 0.996x | 1.380x |
| chess960_perft_d3 | 0.995x | 0.990x | 1.450x |
| polyglot_key | 0.989x | 0.984x | 27.374x |
| polyglot_recompute | 0.982x | 0.978x | 0.999x |
| repetition_key | 0.993x | 0.989x | 18.023x |
| corpus_count | 1.004x | 0.994x | 1.186x |
| corpus_materialize | 0.982x | 0.981x | 1.087x |
| corpus_uci_sorted | 0.995x | 0.995x | 1.782x |
| replay_trusted_native | 1.028x | 1.019x | 0.809x |
| replay_checked_native | 1.001x | 0.999x | 1.065x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.03x).

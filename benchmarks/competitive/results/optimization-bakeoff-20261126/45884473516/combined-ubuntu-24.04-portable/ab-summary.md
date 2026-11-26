| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.990x | 1.324x |
| materialize | 1.039x | 1.018x | 1.360x |
| perft_bulk_d4 | 1.001x | 0.993x | 1.309x |
| perft_full_d3 | 0.997x | 0.970x | 1.270x |
| fen_parse | 0.990x | 0.964x | 1.136x |
| fen_write_native_ep | 1.040x | 1.005x | 1.662x |
| snapshot | 1.000x | 0.997x | 0.987x |
| san_parse_all | 1.010x | 0.988x | 2.013x |
| san_render_native | 1.029x | 0.991x | 2.292x |
| apply_children_full | 0.999x | 0.981x | 1.407x |
| make_unmake_full | 1.001x | 0.962x | 1.449x |
| startpos | 0.999x | 0.999x | 1.305x |
| chess960_materialize | 1.012x | 0.995x | 1.491x |
| chess960_perft_d3 | 1.018x | 1.004x | 1.579x |
| polyglot_key | 0.994x | 0.983x | 23.628x |
| polyglot_recompute | 0.996x | 0.995x | 1.010x |
| repetition_key | 0.997x | 0.971x | 17.845x |
| corpus_count | 1.017x | 1.012x | 1.315x |
| corpus_materialize | 1.012x | 1.010x | 1.217x |
| corpus_uci_sorted | 0.999x | 0.992x | 1.732x |
| replay_trusted_native | 0.934x | 0.918x | 0.834x |
| replay_checked_native | 0.958x | 0.937x | 1.092x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.95x).

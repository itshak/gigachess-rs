| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.989x | 0.960x | 1.533x |
| materialize | 0.912x | 0.853x | 1.463x |
| perft_bulk_d4 | 0.969x | 0.958x | 1.157x |
| perft_full_d3 | 0.892x | 0.825x | 1.263x |
| fen_parse | 1.018x | 0.941x | 1.057x |
| fen_write_native_ep | 0.957x | 0.925x | 1.800x |
| snapshot | 0.999x | 0.997x | 1.153x |
| san_parse_all | 1.006x | 0.950x | 1.910x |
| san_render_native | 0.928x | 0.877x | 1.958x |
| apply_children_full | 0.974x | 0.948x | 1.270x |
| make_unmake_full | 0.892x | 0.809x | 1.040x |
| startpos | 0.744x | 0.744x | 0.831x |
| chess960_materialize | 0.909x | 0.841x | 1.381x |
| chess960_perft_d3 | 0.933x | 0.876x | 1.375x |
| polyglot_key | 0.994x | 0.989x | 39.306x |
| polyglot_recompute | 1.043x | 1.041x | 1.020x |
| repetition_key | 1.159x | 1.022x | 22.828x |
| corpus_count | 1.061x | 1.003x | 1.128x |
| corpus_materialize | 0.997x | 0.996x | 1.079x |
| corpus_uci_sorted | 0.995x | 0.983x | 1.550x |
| replay_trusted_native | 0.937x | 0.885x | 0.837x |
| replay_checked_native | 0.959x | 0.934x | 1.026x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.92x).

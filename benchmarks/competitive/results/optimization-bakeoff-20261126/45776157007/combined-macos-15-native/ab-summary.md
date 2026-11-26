| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.889x | 0.514x | 1.234x |
| materialize | 0.928x | 0.631x | 0.963x |
| perft_bulk_d4 | 0.915x | 0.617x | 1.214x |
| perft_full_d3 | 0.954x | 0.758x | 0.771x |
| fen_parse | 0.829x | 0.745x | 1.098x |
| fen_write_native_ep | 1.022x | 0.848x | 1.269x |
| snapshot | 0.884x | 0.636x | 0.773x |
| san_parse_all | 1.080x | 0.921x | 1.769x |
| san_render_native | 0.952x | 0.843x | 2.780x |
| apply_children_full | 0.919x | 0.701x | 0.761x |
| make_unmake_full | 1.036x | 0.861x | 1.095x |
| startpos | 0.972x | 0.972x | 2.144x |
| chess960_materialize | 1.169x | 1.062x | 1.260x |
| chess960_perft_d3 | 1.000x | 0.938x | 1.153x |
| polyglot_key | 0.910x | 0.798x | 36.086x |
| polyglot_recompute | 1.170x | 1.040x | 0.993x |
| repetition_key | 1.231x | 1.129x | 19.352x |
| corpus_count | 1.120x | 1.087x | 1.381x |
| corpus_materialize | 1.095x | 1.003x | 1.376x |
| corpus_uci_sorted | 1.272x | 1.169x | 1.156x |
| replay_trusted_native | 0.889x | 0.875x | 0.469x |
| replay_checked_native | 0.818x | 0.728x | 0.786x |

Cold first legal move list: 0.014 ms -> 0.014 ms (0.96x).

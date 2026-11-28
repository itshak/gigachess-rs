| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.784x | 0.681x | 1.435x |
| materialize | 0.860x | 0.682x | 1.178x |
| perft_bulk_d4 | 0.859x | 0.707x | 1.220x |
| perft_full_d3 | 0.942x | 0.834x | 1.350x |
| fen_parse | 0.915x | 0.752x | 1.353x |
| fen_write_native_ep | 0.845x | 0.634x | 1.612x |
| snapshot | 0.793x | 0.565x | 0.821x |
| san_parse_all | 0.993x | 0.836x | 1.958x |
| san_render_native | 0.980x | 0.753x | 3.131x |
| apply_children_full | 0.869x | 0.725x | 1.068x |
| make_unmake_full | 1.016x | 0.841x | 1.405x |
| startpos | 0.941x | 0.941x | 0.771x |
| chess960_materialize | 1.047x | 0.876x | 1.601x |
| chess960_perft_d3 | 1.096x | 0.921x | 1.543x |
| polyglot_key | 0.982x | 0.963x | 42.083x |
| polyglot_recompute | 1.577x | 1.450x | 1.631x |
| repetition_key | 0.833x | 0.726x | 21.698x |
| corpus_count | 1.072x | 0.987x | 1.201x |
| corpus_materialize | 0.919x | 0.902x | 1.321x |
| corpus_uci_sorted | 0.854x | 0.841x | 1.239x |
| replay_trusted_native | 0.822x | 0.723x | 0.631x |
| replay_checked_native | 0.755x | 0.726x | 1.373x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.80x).

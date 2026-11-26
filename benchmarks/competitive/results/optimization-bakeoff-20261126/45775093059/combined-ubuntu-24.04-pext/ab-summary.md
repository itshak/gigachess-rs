| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.173x | 1.118x | 1.457x |
| materialize | 0.999x | 0.980x | 1.499x |
| perft_bulk_d4 | 1.407x | 1.340x | 1.237x |
| perft_full_d3 | 2.545x | 2.312x | 1.505x |
| fen_parse | 0.960x | 0.910x | 1.006x |
| fen_write_native_ep | 0.996x | 0.986x | 1.336x |
| snapshot | 2.506x | 2.491x | 1.327x |
| san_parse_all | 1.930x | 1.878x | 1.900x |
| san_render_native | 2.094x | 1.949x | 2.203x |
| apply_children_full | 1.348x | 1.317x | 0.790x |
| make_unmake_full | 1.331x | 1.232x | 0.985x |
| startpos | 1.978x | 1.978x | 1.250x |
| chess960_materialize | 1.190x | 1.087x | 1.440x |
| chess960_perft_d3 | 1.446x | 1.331x | 1.332x |
| polyglot_key | 1.072x | 0.997x | 39.153x |
| polyglot_recompute | 0.895x | 0.877x | 0.873x |
| repetition_key | 1.176x | 0.889x | 19.053x |
| corpus_count | 1.198x | 1.111x | 1.372x |
| corpus_materialize | 1.029x | 1.027x | 1.126x |
| corpus_uci_sorted | 1.453x | 1.434x | 0.928x |
| replay_trusted_native | 1.310x | 1.204x | 0.639x |
| replay_checked_native | 1.349x | 1.153x | 1.002x |

Cold first legal move list: 0.978 ms -> 0.007 ms (133.46x).

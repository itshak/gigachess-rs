| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.964x | 1.557x |
| materialize | 0.937x | 0.891x | 1.523x |
| perft_bulk_d4 | 1.006x | 0.986x | 1.194x |
| perft_full_d3 | 1.010x | 0.996x | 1.378x |
| fen_parse | 1.006x | 0.988x | 1.067x |
| fen_write_native_ep | 0.998x | 0.987x | 2.152x |
| snapshot | 1.002x | 0.999x | 1.153x |
| san_parse_all | 1.024x | 0.981x | 1.876x |
| san_render_native | 0.992x | 0.975x | 2.137x |
| apply_children_full | 0.997x | 0.974x | 1.337x |
| make_unmake_full | 0.926x | 0.892x | 1.112x |
| startpos | 0.897x | 0.897x | 1.118x |
| chess960_materialize | 1.016x | 0.995x | 1.517x |
| chess960_perft_d3 | 1.004x | 1.002x | 1.486x |
| polyglot_key | 0.878x | 0.768x | 33.896x |
| polyglot_recompute | 1.024x | 1.018x | 1.026x |
| repetition_key | 0.758x | 0.701x | 18.015x |
| corpus_count | 0.927x | 0.871x | 1.046x |
| corpus_materialize | 1.004x | 1.001x | 1.078x |
| corpus_uci_sorted | 1.007x | 1.004x | 1.559x |
| replay_trusted_native | 0.919x | 0.844x | 0.839x |
| replay_checked_native | 0.952x | 0.944x | 1.063x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).

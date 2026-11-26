| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.866x | 1.297x |
| materialize | 0.924x | 0.848x | 1.128x |
| perft_bulk_d4 | 0.997x | 0.975x | 1.368x |
| perft_full_d3 | 0.964x | 0.866x | 1.255x |
| fen_parse | 1.010x | 0.976x | 1.288x |
| fen_write_native_ep | 0.972x | 0.965x | 0.993x |
| snapshot | 0.483x | 0.477x | 1.081x |
| san_parse_all | 1.033x | 0.976x | 2.078x |
| san_render_native | 0.976x | 0.891x | 3.793x |
| apply_children_full | 0.954x | 0.911x | 1.146x |
| make_unmake_full | 0.966x | 0.940x | 1.200x |
| startpos | 1.001x | 1.001x | 2.174x |
| chess960_materialize | 0.950x | 0.938x | 1.227x |
| chess960_perft_d3 | 0.919x | 0.847x | 1.249x |
| polyglot_key | 0.901x | 0.877x | 23.589x |
| polyglot_recompute | 0.969x | 0.945x | 0.996x |
| repetition_key | 1.015x | 0.983x | 19.837x |
| corpus_count | 0.954x | 0.950x | 1.358x |
| corpus_materialize | 0.971x | 0.947x | 1.209x |
| corpus_uci_sorted | 1.009x | 1.000x | 1.533x |
| replay_trusted_native | 0.999x | 0.983x | 0.690x |
| replay_checked_native | 1.006x | 0.997x | 0.999x |

Cold first legal move list: 0.029 ms -> 0.027 ms (1.05x).

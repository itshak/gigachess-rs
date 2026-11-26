| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.458x |
| materialize | 1.002x | 0.979x | 1.250x |
| perft_bulk_d4 | 0.937x | 0.910x | 1.291x |
| perft_full_d3 | 0.996x | 0.983x | 1.278x |
| fen_parse | 1.114x | 1.079x | 1.205x |
| fen_write_native_ep | 0.994x | 0.983x | 1.703x |
| snapshot | 0.779x | 0.776x | 0.973x |
| san_parse_all | 0.985x | 0.975x | 1.957x |
| san_render_native | 0.969x | 0.943x | 2.126x |
| apply_children_full | 0.954x | 0.904x | 1.259x |
| make_unmake_full | 0.982x | 0.954x | 1.225x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 1.031x | 1.026x | 1.423x |
| chess960_perft_d3 | 0.958x | 0.938x | 1.459x |
| polyglot_key | 0.893x | 0.803x | 25.317x |
| polyglot_recompute | 0.976x | 0.974x | 1.026x |
| repetition_key | 0.930x | 0.860x | 19.360x |
| corpus_count | 0.982x | 0.969x | 1.358x |
| corpus_materialize | 1.007x | 0.999x | 1.171x |
| corpus_uci_sorted | 0.994x | 0.985x | 1.756x |
| replay_trusted_native | 0.943x | 0.849x | 0.849x |
| replay_checked_native | 0.993x | 0.989x | 1.062x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.00x).

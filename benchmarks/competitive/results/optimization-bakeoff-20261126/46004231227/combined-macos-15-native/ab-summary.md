| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.920x | 0.823x | 1.425x |
| materialize | 0.981x | 0.598x | 1.156x |
| perft_bulk_d4 | 1.077x | 0.901x | 1.240x |
| perft_full_d3 | 1.081x | 0.881x | 1.332x |
| fen_parse | 1.060x | 0.872x | 1.140x |
| fen_write_native_ep | 1.012x | 0.846x | 1.682x |
| snapshot | 0.915x | 0.620x | 0.825x |
| san_parse_all | 0.995x | 0.899x | 1.732x |
| san_render_native | 1.049x | 0.872x | 3.145x |
| apply_children_full | 1.022x | 0.911x | 1.084x |
| make_unmake_full | 0.982x | 0.608x | 1.362x |
| startpos | 1.033x | 1.033x | 1.574x |
| chess960_materialize | 1.130x | 1.060x | 1.315x |
| chess960_perft_d3 | 1.443x | 1.214x | 1.951x |
| polyglot_key | 1.700x | 1.636x | 40.398x |
| polyglot_recompute | 1.228x | 1.144x | 0.926x |
| repetition_key | 1.268x | 1.185x | 20.636x |
| corpus_count | 1.170x | 0.993x | 1.216x |
| corpus_materialize | 1.025x | 0.940x | 1.050x |
| corpus_uci_sorted | 1.238x | 1.220x | 1.239x |
| replay_trusted_native | 1.292x | 1.038x | 0.629x |
| replay_checked_native | 1.360x | 1.161x | 1.380x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.11x).

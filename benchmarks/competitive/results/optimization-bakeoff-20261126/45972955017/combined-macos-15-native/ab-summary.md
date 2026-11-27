| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.880x | 1.575x |
| materialize | 1.018x | 0.893x | 1.066x |
| perft_bulk_d4 | 1.120x | 0.911x | 1.273x |
| perft_full_d3 | 0.980x | 0.769x | 1.276x |
| fen_parse | 1.097x | 0.804x | 1.181x |
| fen_write_native_ep | 1.084x | 0.889x | 1.777x |
| snapshot | 1.036x | 0.913x | 0.991x |
| san_parse_all | 1.169x | 0.885x | 1.749x |
| san_render_native | 1.205x | 0.806x | 2.740x |
| apply_children_full | 1.234x | 1.008x | 1.157x |
| make_unmake_full | 1.069x | 0.930x | 1.281x |
| startpos | 0.962x | 0.962x | 1.232x |
| chess960_materialize | 0.931x | 0.906x | 1.489x |
| chess960_perft_d3 | 1.011x | 0.936x | 1.458x |
| polyglot_key | 1.042x | 1.012x | 38.267x |
| polyglot_recompute | 1.089x | 1.053x | 1.039x |
| repetition_key | 1.033x | 0.982x | 21.233x |
| corpus_count | 1.079x | 1.020x | 1.189x |
| corpus_materialize | 1.073x | 1.012x | 1.151x |
| corpus_uci_sorted | 1.203x | 1.174x | 1.413x |
| replay_trusted_native | 1.187x | 1.169x | 0.647x |
| replay_checked_native | 1.022x | 0.978x | 1.298x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.063x | 0.989x | 1.610x |
| materialize | 0.941x | 0.817x | 1.010x |
| perft_bulk_d4 | 1.066x | 0.943x | 1.226x |
| perft_full_d3 | 1.085x | 0.901x | 1.417x |
| fen_parse | 1.029x | 0.916x | 1.281x |
| fen_write_native_ep | 0.943x | 0.850x | 1.663x |
| snapshot | 1.082x | 0.970x | 1.044x |
| san_parse_all | 1.018x | 0.897x | 1.687x |
| san_render_native | 1.053x | 0.931x | 3.008x |
| apply_children_full | 0.871x | 0.755x | 0.981x |
| make_unmake_full | 1.019x | 0.835x | 1.256x |
| startpos | 1.140x | 1.140x | 1.205x |
| chess960_materialize | 0.753x | 0.669x | 1.189x |
| chess960_perft_d3 | 0.952x | 0.908x | 1.499x |
| polyglot_key | 1.117x | 1.038x | 38.132x |
| polyglot_recompute | 1.693x | 1.309x | 1.538x |
| repetition_key | 1.342x | 1.107x | 19.598x |
| corpus_count | 1.269x | 1.128x | 1.327x |
| corpus_materialize | 1.119x | 1.028x | 1.161x |
| corpus_uci_sorted | 1.160x | 0.941x | 1.727x |
| replay_trusted_native | 1.019x | 0.914x | 0.716x |
| replay_checked_native | 1.069x | 0.960x | 1.263x |

Cold first legal move list: 0.002 ms -> 0.004 ms (0.61x).

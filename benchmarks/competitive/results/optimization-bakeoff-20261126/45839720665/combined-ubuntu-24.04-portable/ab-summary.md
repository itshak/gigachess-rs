| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.023x | 1.000x | 1.256x |
| materialize | 1.013x | 0.972x | 1.317x |
| perft_bulk_d4 | 0.985x | 0.907x | 1.175x |
| perft_full_d3 | 1.051x | 0.947x | 1.079x |
| fen_parse | 1.006x | 0.991x | 1.193x |
| fen_write_native_ep | 1.033x | 0.984x | 1.540x |
| snapshot | 0.998x | 0.965x | 0.991x |
| san_parse_all | 0.989x | 0.959x | 1.762x |
| san_render_native | 1.005x | 0.964x | 2.224x |
| apply_children_full | 1.072x | 0.867x | 1.235x |
| make_unmake_full | 1.044x | 1.009x | 1.317x |
| startpos | 1.020x | 1.020x | 1.280x |
| chess960_materialize | 1.000x | 0.999x | 1.539x |
| chess960_perft_d3 | 1.060x | 1.058x | 1.508x |
| polyglot_key | 0.976x | 0.926x | 32.155x |
| polyglot_recompute | 0.980x | 0.975x | 1.022x |
| repetition_key | 1.178x | 1.066x | 24.353x |
| corpus_count | 1.025x | 1.010x | 1.128x |
| corpus_materialize | 0.974x | 0.952x | 1.172x |
| corpus_uci_sorted | 0.974x | 0.969x | 1.595x |
| replay_trusted_native | 1.107x | 1.024x | 0.776x |
| replay_checked_native | 1.076x | 1.040x | 1.290x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.10x).

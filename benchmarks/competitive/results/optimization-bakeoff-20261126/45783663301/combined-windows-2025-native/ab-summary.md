| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.869x | 0.473x | 1.133x |
| materialize | 0.980x | 0.899x | 1.134x |
| perft_bulk_d4 | 0.963x | 0.889x | 1.215x |
| perft_full_d3 | 0.979x | 0.895x | 0.974x |
| fen_parse | 0.959x | 0.747x | 1.305x |
| fen_write_native_ep | 1.007x | 0.997x | 1.262x |
| snapshot | 1.241x | 1.227x | 1.238x |
| san_parse_all | 0.989x | 0.920x | 1.959x |
| san_render_native | 1.006x | 0.985x | 3.683x |
| apply_children_full | 0.983x | 0.941x | 1.090x |
| make_unmake_full | 0.988x | 0.932x | 1.251x |
| startpos | 1.023x | 1.023x | 1.255x |
| chess960_materialize | 1.055x | 1.039x | 1.171x |
| chess960_perft_d3 | 0.900x | 0.801x | 1.179x |
| polyglot_key | 1.062x | 1.051x | 38.014x |
| polyglot_recompute | 1.065x | 1.019x | 0.975x |
| repetition_key | 0.965x | 0.899x | 25.433x |
| corpus_count | 0.612x | 0.539x | 0.703x |
| corpus_materialize | 1.089x | 1.066x | 1.194x |
| corpus_uci_sorted | 1.036x | 1.021x | 1.384x |
| replay_trusted_native | 1.005x | 1.000x | 0.657x |
| replay_checked_native | 1.054x | 1.025x | 0.919x |

Cold first legal move list: 0.020 ms -> 0.020 ms (0.99x).

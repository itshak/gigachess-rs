| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.999x | 1.285x |
| materialize | 0.987x | 0.963x | 1.202x |
| perft_bulk_d4 | 1.007x | 0.973x | 1.318x |
| perft_full_d3 | 0.989x | 0.930x | 1.213x |
| fen_parse | 1.055x | 1.020x | 1.212x |
| fen_write_native_ep | 0.984x | 0.918x | 1.701x |
| snapshot | 0.743x | 0.713x | 0.844x |
| san_parse_all | 0.959x | 0.949x | 2.144x |
| san_render_native | 0.983x | 0.966x | 2.203x |
| apply_children_full | 1.025x | 0.990x | 1.168x |
| make_unmake_full | 1.019x | 0.993x | 1.259x |
| startpos | 0.926x | 0.926x | 1.142x |
| chess960_materialize | 0.995x | 0.988x | 1.297x |
| chess960_perft_d3 | 0.978x | 0.974x | 1.314x |
| polyglot_key | 0.938x | 0.928x | 24.487x |
| polyglot_recompute | 1.022x | 1.018x | 1.057x |
| repetition_key | 0.950x | 0.908x | 19.334x |
| corpus_count | 1.004x | 1.000x | 1.367x |
| corpus_materialize | 1.010x | 1.007x | 1.215x |
| corpus_uci_sorted | 0.985x | 0.980x | 1.685x |
| replay_trusted_native | 1.026x | 1.016x | 0.716x |
| replay_checked_native | 0.986x | 0.985x | 1.010x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.00x).

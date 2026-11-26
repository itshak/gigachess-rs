| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.932x | 0.732x | 1.151x |
| materialize | 1.022x | 1.000x | 1.398x |
| perft_bulk_d4 | 1.015x | 0.995x | 1.216x |
| perft_full_d3 | 1.001x | 0.982x | 1.114x |
| fen_parse | 1.105x | 1.045x | 1.270x |
| fen_write_native_ep | 1.004x | 0.961x | 1.466x |
| snapshot | 1.001x | 0.992x | 1.000x |
| san_parse_all | 1.031x | 0.977x | 1.747x |
| san_render_native | 0.961x | 0.804x | 2.210x |
| apply_children_full | 0.993x | 0.951x | 1.191x |
| make_unmake_full | 1.016x | 1.000x | 1.339x |
| startpos | 0.999x | 0.999x | 1.287x |
| chess960_materialize | 1.022x | 1.017x | 1.613x |
| chess960_perft_d3 | 1.006x | 0.989x | 1.471x |
| polyglot_key | 1.017x | 1.003x | 32.028x |
| polyglot_recompute | 0.994x | 0.987x | 1.004x |
| repetition_key | 1.214x | 1.144x | 24.914x |
| corpus_count | 0.979x | 0.962x | 1.066x |
| corpus_materialize | 1.022x | 1.021x | 1.233x |
| corpus_uci_sorted | 1.002x | 0.994x | 1.619x |
| replay_trusted_native | 0.994x | 0.979x | 0.669x |
| replay_checked_native | 1.014x | 0.994x | 1.223x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.298x |
| materialize | 1.000x | 0.996x | 1.203x |
| perft_bulk_d4 | 0.955x | 0.924x | 1.215x |
| perft_full_d3 | 0.931x | 0.895x | 0.699x |
| fen_parse | 0.904x | 0.875x | 1.118x |
| fen_write_native_ep | 0.996x | 0.989x | 1.058x |
| snapshot | 0.995x | 0.993x | 0.870x |
| san_parse_all | 1.008x | 0.965x | 1.773x |
| san_render_native | 0.903x | 0.855x | 2.167x |
| apply_children_full | 0.867x | 0.808x | 0.734x |
| make_unmake_full | 0.823x | 0.803x | 0.947x |
| startpos | 0.569x | 0.569x | 0.674x |
| chess960_materialize | 1.002x | 1.002x | 1.242x |
| chess960_perft_d3 | 0.942x | 0.929x | 1.150x |
| polyglot_key | 0.800x | 0.740x | 20.728x |
| polyglot_recompute | 1.002x | 0.994x | 1.073x |
| repetition_key | 0.797x | 0.713x | 15.347x |
| corpus_count | 0.977x | 0.956x | 1.362x |
| corpus_materialize | 0.995x | 0.989x | 1.179x |
| corpus_uci_sorted | 1.000x | 1.000x | 0.971x |
| replay_trusted_native | 0.855x | 0.853x | 0.346x |
| replay_checked_native | 0.882x | 0.874x | 0.796x |

Cold first legal move list: 0.013 ms -> 0.011 ms (1.10x).

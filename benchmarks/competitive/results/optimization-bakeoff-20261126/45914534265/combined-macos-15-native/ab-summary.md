| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.546x | 1.467x |
| materialize | 0.973x | 0.723x | 1.047x |
| perft_bulk_d4 | 1.028x | 0.828x | 1.338x |
| perft_full_d3 | 1.023x | 0.773x | 1.262x |
| fen_parse | 1.072x | 0.876x | 1.182x |
| fen_write_native_ep | 1.004x | 0.855x | 1.675x |
| snapshot | 1.008x | 0.834x | 0.969x |
| san_parse_all | 0.962x | 0.862x | 1.672x |
| san_render_native | 1.112x | 0.954x | 2.857x |
| apply_children_full | 1.128x | 1.002x | 1.073x |
| make_unmake_full | 1.101x | 0.901x | 1.312x |
| startpos | 1.146x | 1.146x | 1.206x |
| chess960_materialize | 1.018x | 0.870x | 1.659x |
| chess960_perft_d3 | 0.913x | 0.731x | 1.836x |
| polyglot_key | 1.014x | 0.864x | 40.101x |
| polyglot_recompute | 0.813x | 0.774x | 0.959x |
| repetition_key | 0.894x | 0.749x | 21.065x |
| corpus_count | 0.858x | 0.736x | 1.244x |
| corpus_materialize | 0.732x | 0.663x | 1.105x |
| corpus_uci_sorted | 0.771x | 0.737x | 1.479x |
| replay_trusted_native | 0.893x | 0.677x | 0.589x |
| replay_checked_native | 0.699x | 0.668x | 1.213x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).

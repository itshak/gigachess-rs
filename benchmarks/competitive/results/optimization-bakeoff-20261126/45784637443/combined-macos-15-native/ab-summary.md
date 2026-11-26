| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.818x | 1.386x |
| materialize | 1.020x | 0.784x | 1.079x |
| perft_bulk_d4 | 0.993x | 0.821x | 1.432x |
| perft_full_d3 | 0.971x | 0.829x | 1.250x |
| fen_parse | 1.026x | 0.758x | 1.157x |
| fen_write_native_ep | 1.062x | 0.896x | 1.865x |
| snapshot | 1.078x | 0.729x | 0.895x |
| san_parse_all | 1.069x | 0.807x | 1.863x |
| san_render_native | 1.018x | 0.870x | 2.777x |
| apply_children_full | 0.932x | 0.823x | 0.913x |
| make_unmake_full | 1.054x | 0.928x | 1.288x |
| startpos | 1.048x | 1.048x | 1.358x |
| chess960_materialize | 0.995x | 0.797x | 1.501x |
| chess960_perft_d3 | 0.936x | 0.809x | 1.409x |
| polyglot_key | 0.791x | 0.736x | 32.783x |
| polyglot_recompute | 0.941x | 0.848x | 1.025x |
| repetition_key | 0.997x | 0.834x | 18.158x |
| corpus_count | 0.935x | 0.880x | 1.340x |
| corpus_materialize | 1.015x | 0.995x | 1.175x |
| corpus_uci_sorted | 0.908x | 0.907x | 1.391x |
| replay_trusted_native | 0.911x | 0.800x | 0.554x |
| replay_checked_native | 1.069x | 0.968x | 0.992x |

Cold first legal move list: 0.021 ms -> 0.009 ms (2.28x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.983x | 1.257x |
| materialize | 1.003x | 0.966x | 1.179x |
| perft_bulk_d4 | 1.012x | 0.986x | 1.231x |
| perft_full_d3 | 0.988x | 0.977x | 0.785x |
| fen_parse | 0.937x | 0.914x | 1.042x |
| fen_write_native_ep | 1.001x | 0.997x | 0.946x |
| snapshot | 1.319x | 1.314x | 1.034x |
| san_parse_all | 1.000x | 0.968x | 1.851x |
| san_render_native | 1.041x | 1.015x | 2.266x |
| apply_children_full | 0.931x | 0.910x | 0.908x |
| make_unmake_full | 1.021x | 0.999x | 0.987x |
| startpos | 1.080x | 1.080x | 1.195x |
| chess960_materialize | 0.982x | 0.978x | 1.219x |
| chess960_perft_d3 | 1.015x | 1.000x | 1.272x |
| polyglot_key | 0.993x | 0.988x | 25.917x |
| polyglot_recompute | 0.993x | 0.981x | 1.051x |
| repetition_key | 0.995x | 0.987x | 20.293x |
| corpus_count | 1.002x | 0.998x | 1.315x |
| corpus_materialize | 1.002x | 0.995x | 1.151x |
| corpus_uci_sorted | 1.725x | 1.691x | 1.657x |
| replay_trusted_native | 0.983x | 0.982x | 0.594x |
| replay_checked_native | 0.992x | 0.979x | 0.963x |

Cold first legal move list: 0.013 ms -> 0.013 ms (0.98x).

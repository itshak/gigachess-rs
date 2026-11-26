| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.999x | 1.366x |
| materialize | 0.993x | 0.966x | 1.270x |
| perft_bulk_d4 | 1.003x | 0.985x | 1.200x |
| perft_full_d3 | 1.053x | 1.001x | 1.277x |
| fen_parse | 1.005x | 0.985x | 1.298x |
| fen_write_native_ep | 1.023x | 1.003x | 1.671x |
| snapshot | 0.990x | 0.966x | 0.995x |
| san_parse_all | 0.965x | 0.810x | 2.043x |
| san_render_native | 0.966x | 0.829x | 3.875x |
| apply_children_full | 0.983x | 0.893x | 1.352x |
| make_unmake_full | 0.998x | 0.988x | 1.387x |
| startpos | 0.999x | 0.999x | 1.250x |
| chess960_materialize | 0.986x | 0.985x | 1.258x |
| chess960_perft_d3 | 1.014x | 1.012x | 1.423x |
| polyglot_key | 0.996x | 0.991x | 29.027x |
| polyglot_recompute | 0.998x | 0.996x | 1.013x |
| repetition_key | 1.014x | 1.007x | 20.792x |
| corpus_count | 0.975x | 0.944x | 1.219x |
| corpus_materialize | 0.987x | 0.984x | 1.082x |
| corpus_uci_sorted | 1.008x | 1.005x | 1.436x |
| replay_trusted_native | 1.007x | 1.000x | 0.831x |
| replay_checked_native | 1.008x | 1.007x | 0.990x |

Cold first legal move list: 0.018 ms -> 0.018 ms (1.02x).

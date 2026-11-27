| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.048x | 0.944x | 1.566x |
| materialize | 1.001x | 0.810x | 1.221x |
| perft_bulk_d4 | 1.047x | 1.013x | 1.453x |
| perft_full_d3 | 1.153x | 1.032x | 1.528x |
| fen_parse | 1.047x | 0.977x | 1.304x |
| fen_write_native_ep | 0.956x | 0.912x | 1.560x |
| snapshot | 1.025x | 0.976x | 0.995x |
| san_parse_all | 1.002x | 0.923x | 2.095x |
| san_render_native | 1.041x | 0.967x | 4.082x |
| apply_children_full | 1.043x | 0.937x | 1.381x |
| make_unmake_full | 0.973x | 0.906x | 1.291x |
| startpos | 0.992x | 0.992x | 1.243x |
| chess960_materialize | 1.015x | 0.979x | 1.383x |
| chess960_perft_d3 | 0.879x | 0.833x | 1.409x |
| polyglot_key | 0.994x | 0.992x | 29.676x |
| polyglot_recompute | 0.923x | 0.913x | 0.973x |
| repetition_key | 1.050x | 1.015x | 24.978x |
| corpus_count | 1.041x | 0.987x | 1.456x |
| corpus_materialize | 1.018x | 1.018x | 1.235x |
| corpus_uci_sorted | 0.991x | 0.990x | 1.430x |
| replay_trusted_native | 0.975x | 0.973x | 0.821x |
| replay_checked_native | 0.953x | 0.905x | 1.089x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.95x).

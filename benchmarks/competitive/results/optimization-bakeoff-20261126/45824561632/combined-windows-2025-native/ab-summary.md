| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.967x | 1.353x |
| materialize | 0.961x | 0.916x | 1.079x |
| perft_bulk_d4 | 1.028x | 0.967x | 1.302x |
| perft_full_d3 | 0.857x | 0.776x | 0.903x |
| fen_parse | 1.032x | 0.992x | 1.314x |
| fen_write_native_ep | 1.013x | 0.998x | 1.633x |
| snapshot | 0.892x | 0.863x | 0.973x |
| san_parse_all | 0.934x | 0.919x | 1.818x |
| san_render_native | 1.013x | 0.977x | 3.386x |
| apply_children_full | 0.954x | 0.911x | 1.252x |
| make_unmake_full | 1.066x | 1.013x | 1.324x |
| startpos | 1.004x | 1.004x | 1.202x |
| chess960_materialize | 0.936x | 0.875x | 1.219x |
| chess960_perft_d3 | 0.967x | 0.925x | 1.421x |
| polyglot_key | 1.044x | 1.042x | 21.064x |
| polyglot_recompute | 1.035x | 1.026x | 1.003x |
| repetition_key | 1.231x | 1.201x | 19.573x |
| corpus_count | 0.988x | 0.978x | 1.173x |
| corpus_materialize | 1.014x | 1.008x | 1.138x |
| corpus_uci_sorted | 1.002x | 0.995x | 1.511x |
| replay_trusted_native | 0.783x | 0.627x | 0.922x |
| replay_checked_native | 0.917x | 0.905x | 0.923x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.09x).

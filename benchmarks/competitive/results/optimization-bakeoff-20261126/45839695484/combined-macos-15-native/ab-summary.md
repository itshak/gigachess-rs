| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.841x | 1.502x |
| materialize | 1.143x | 0.995x | 1.168x |
| perft_bulk_d4 | 1.054x | 0.992x | 1.266x |
| perft_full_d3 | 1.024x | 0.791x | 1.145x |
| fen_parse | 1.097x | 0.818x | 1.275x |
| fen_write_native_ep | 1.058x | 0.761x | 1.806x |
| snapshot | 0.933x | 0.471x | 0.839x |
| san_parse_all | 1.027x | 0.501x | 1.793x |
| san_render_native | 1.071x | 0.888x | 2.660x |
| apply_children_full | 1.049x | 0.730x | 1.090x |
| make_unmake_full | 1.039x | 0.835x | 1.324x |
| startpos | 1.432x | 1.432x | 1.275x |
| chess960_materialize | 0.923x | 0.904x | 1.660x |
| chess960_perft_d3 | 0.841x | 0.703x | 1.418x |
| polyglot_key | 0.816x | 0.735x | 37.395x |
| polyglot_recompute | 0.813x | 0.638x | 1.027x |
| repetition_key | 0.885x | 0.669x | 20.389x |
| corpus_count | 0.829x | 0.711x | 1.336x |
| corpus_materialize | 1.161x | 1.021x | 1.086x |
| corpus_uci_sorted | 1.026x | 0.931x | 1.533x |
| replay_trusted_native | 0.995x | 0.923x | 0.670x |
| replay_checked_native | 0.758x | 0.707x | 1.240x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.89x).

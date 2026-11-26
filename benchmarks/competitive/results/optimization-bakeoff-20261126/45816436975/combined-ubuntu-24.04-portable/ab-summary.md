| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.978x | 1.214x |
| materialize | 1.043x | 0.986x | 1.591x |
| perft_bulk_d4 | 0.917x | 0.879x | 1.070x |
| perft_full_d3 | 0.958x | 0.856x | 1.444x |
| fen_parse | 1.037x | 1.010x | 1.197x |
| fen_write_native_ep | 0.989x | 0.952x | 1.912x |
| snapshot | 0.998x | 0.994x | 0.997x |
| san_parse_all | 0.909x | 0.867x | 1.728x |
| san_render_native | 0.977x | 0.957x | 2.103x |
| apply_children_full | 0.988x | 0.969x | 1.205x |
| make_unmake_full | 1.006x | 0.975x | 1.193x |
| startpos | 0.999x | 0.999x | 1.433x |
| chess960_materialize | 0.943x | 0.888x | 1.286x |
| chess960_perft_d3 | 1.040x | 0.983x | 1.383x |
| polyglot_key | 1.015x | 0.995x | 43.248x |
| polyglot_recompute | 1.076x | 1.052x | 1.122x |
| repetition_key | 0.792x | 0.717x | 18.460x |
| corpus_count | 0.780x | 0.738x | 0.988x |
| corpus_materialize | 0.935x | 0.917x | 1.087x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.550x |
| replay_trusted_native | 0.988x | 0.957x | 0.832x |
| replay_checked_native | 1.006x | 0.966x | 1.097x |

Cold first legal move list: 0.010 ms -> 0.005 ms (1.87x).

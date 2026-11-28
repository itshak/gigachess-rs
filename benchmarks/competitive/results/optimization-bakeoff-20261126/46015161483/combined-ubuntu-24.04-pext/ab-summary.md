| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.042x | 0.929x | 1.611x |
| materialize | 1.026x | 0.941x | 1.643x |
| perft_bulk_d4 | 1.046x | 1.010x | 1.255x |
| perft_full_d3 | 1.190x | 0.974x | 1.638x |
| fen_parse | 0.991x | 0.947x | 1.161x |
| fen_write_native_ep | 0.975x | 0.917x | 1.974x |
| snapshot | 0.953x | 0.886x | 1.170x |
| san_parse_all | 0.960x | 0.875x | 1.687x |
| san_render_native | 0.962x | 0.923x | 2.189x |
| apply_children_full | 0.967x | 0.914x | 1.335x |
| make_unmake_full | 0.970x | 0.917x | 1.155x |
| startpos | 1.081x | 1.081x | 1.118x |
| chess960_materialize | 1.076x | 1.066x | 1.485x |
| chess960_perft_d3 | 1.048x | 0.990x | 1.493x |
| polyglot_key | 0.928x | 0.859x | 38.152x |
| polyglot_recompute | 1.550x | 1.220x | 1.598x |
| repetition_key | 0.905x | 0.886x | 18.819x |
| corpus_count | 1.176x | 1.133x | 1.240x |
| corpus_materialize | 1.007x | 1.004x | 1.097x |
| corpus_uci_sorted | 0.952x | 0.916x | 1.505x |
| replay_trusted_native | 1.033x | 1.008x | 0.897x |
| replay_checked_native | 1.023x | 0.991x | 1.102x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.19x).

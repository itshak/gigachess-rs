| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.429x |
| materialize | 0.999x | 0.995x | 1.528x |
| perft_bulk_d4 | 0.995x | 0.956x | 1.047x |
| perft_full_d3 | 1.000x | 0.998x | 1.472x |
| fen_parse | 0.942x | 0.842x | 0.981x |
| fen_write_native_ep | 0.995x | 0.971x | 2.132x |
| snapshot | 1.001x | 0.996x | 1.154x |
| san_parse_all | 0.988x | 0.981x | 1.887x |
| san_render_native | 1.004x | 1.000x | 2.130x |
| apply_children_full | 0.979x | 0.967x | 1.266x |
| make_unmake_full | 1.004x | 0.954x | 1.171x |
| startpos | 0.994x | 0.994x | 1.113x |
| chess960_materialize | 1.008x | 1.004x | 1.376x |
| chess960_perft_d3 | 0.967x | 0.941x | 1.337x |
| polyglot_key | 1.021x | 1.001x | 38.730x |
| polyglot_recompute | 1.003x | 1.001x | 1.025x |
| repetition_key | 0.975x | 0.954x | 18.231x |
| corpus_count | 0.999x | 0.998x | 0.971x |
| corpus_materialize | 1.001x | 0.999x | 1.060x |
| corpus_uci_sorted | 1.014x | 1.010x | 1.559x |
| replay_trusted_native | 0.980x | 0.931x | 0.877x |
| replay_checked_native | 1.001x | 0.956x | 1.113x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).

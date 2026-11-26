| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.986x | 1.416x |
| materialize | 0.996x | 0.990x | 1.155x |
| perft_bulk_d4 | 1.024x | 1.001x | 1.275x |
| perft_full_d3 | 0.993x | 0.960x | 0.982x |
| fen_parse | 1.087x | 1.059x | 1.166x |
| fen_write_native_ep | 0.996x | 0.975x | 1.809x |
| snapshot | 0.993x | 0.986x | 1.251x |
| san_parse_all | 0.966x | 0.960x | 2.123x |
| san_render_native | 0.973x | 0.958x | 2.044x |
| apply_children_full | 0.935x | 0.899x | 1.229x |
| make_unmake_full | 1.052x | 1.018x | 1.231x |
| startpos | 1.006x | 1.006x | 1.201x |
| chess960_materialize | 1.018x | 1.013x | 1.518x |
| chess960_perft_d3 | 0.998x | 0.993x | 1.482x |
| polyglot_key | 0.979x | 0.958x | 20.467x |
| polyglot_recompute | 0.945x | 0.938x | 0.972x |
| repetition_key | 1.140x | 1.134x | 17.627x |
| corpus_count | 0.999x | 0.989x | 1.252x |
| corpus_materialize | 0.988x | 0.978x | 1.234x |
| corpus_uci_sorted | 0.989x | 0.982x | 1.662x |
| replay_trusted_native | 1.043x | 0.996x | 1.125x |
| replay_checked_native | 1.001x | 1.000x | 1.016x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).

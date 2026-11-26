| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.967x | 1.429x |
| materialize | 1.061x | 0.990x | 1.173x |
| perft_bulk_d4 | 0.985x | 0.951x | 1.293x |
| perft_full_d3 | 0.975x | 0.950x | 0.965x |
| fen_parse | 0.995x | 0.929x | 1.069x |
| fen_write_native_ep | 1.003x | 0.996x | 1.843x |
| snapshot | 1.230x | 1.206x | 1.247x |
| san_parse_all | 0.998x | 0.982x | 2.237x |
| san_render_native | 0.975x | 0.873x | 2.108x |
| apply_children_full | 0.966x | 0.899x | 1.019x |
| make_unmake_full | 1.020x | 0.999x | 1.222x |
| startpos | 1.094x | 1.094x | 1.198x |
| chess960_materialize | 1.059x | 1.051x | 1.488x |
| chess960_perft_d3 | 0.991x | 0.985x | 1.496x |
| polyglot_key | 1.002x | 1.001x | 20.364x |
| polyglot_recompute | 1.037x | 1.034x | 1.023x |
| repetition_key | 1.000x | 0.998x | 19.248x |
| corpus_count | 0.991x | 0.988x | 1.449x |
| corpus_materialize | 1.002x | 1.000x | 1.266x |
| corpus_uci_sorted | 1.003x | 0.994x | 1.686x |
| replay_trusted_native | 1.058x | 1.043x | 0.758x |
| replay_checked_native | 0.988x | 0.985x | 0.923x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.09x).

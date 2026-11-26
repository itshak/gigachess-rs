| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.170x | 0.953x | 1.540x |
| materialize | 1.028x | 0.952x | 1.078x |
| perft_bulk_d4 | 1.084x | 1.005x | 1.215x |
| perft_full_d3 | 0.910x | 0.549x | 1.246x |
| fen_parse | 0.988x | 0.972x | 1.283x |
| fen_write_native_ep | 0.990x | 0.977x | 1.763x |
| snapshot | 0.989x | 0.966x | 1.013x |
| san_parse_all | 1.005x | 0.966x | 1.764x |
| san_render_native | 0.973x | 0.919x | 2.621x |
| apply_children_full | 0.967x | 0.950x | 1.121x |
| make_unmake_full | 1.016x | 0.964x | 1.298x |
| startpos | 1.409x | 1.409x | 1.190x |
| chess960_materialize | 1.101x | 1.083x | 1.577x |
| chess960_perft_d3 | 1.247x | 1.113x | 1.525x |
| polyglot_key | 1.029x | 1.007x | 38.696x |
| polyglot_recompute | 1.029x | 0.995x | 0.980x |
| repetition_key | 1.044x | 1.032x | 19.836x |
| corpus_count | 1.200x | 1.111x | 1.132x |
| corpus_materialize | 1.120x | 1.015x | 1.134x |
| corpus_uci_sorted | 1.016x | 0.997x | 1.407x |
| replay_trusted_native | 1.005x | 0.973x | 0.651x |
| replay_checked_native | 0.975x | 0.952x | 1.157x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.20x).

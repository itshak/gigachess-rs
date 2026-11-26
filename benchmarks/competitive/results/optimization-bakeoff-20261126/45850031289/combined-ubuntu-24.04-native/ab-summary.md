| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.977x | 1.501x |
| materialize | 1.003x | 0.994x | 1.577x |
| perft_bulk_d4 | 0.984x | 0.970x | 1.157x |
| perft_full_d3 | 0.995x | 0.989x | 1.335x |
| fen_parse | 1.008x | 0.996x | 1.180x |
| fen_write_native_ep | 1.009x | 0.986x | 1.900x |
| snapshot | 1.006x | 0.999x | 1.332x |
| san_parse_all | 1.009x | 1.004x | 1.736x |
| san_render_native | 1.018x | 1.009x | 2.144x |
| apply_children_full | 1.024x | 1.011x | 1.258x |
| make_unmake_full | 0.996x | 0.958x | 1.168x |
| startpos | 1.121x | 1.121x | 1.263x |
| chess960_materialize | 0.954x | 0.949x | 1.462x |
| chess960_perft_d3 | 1.006x | 1.004x | 1.510x |
| polyglot_key | 1.011x | 1.004x | 40.212x |
| polyglot_recompute | 0.999x | 0.992x | 1.028x |
| repetition_key | 0.818x | 0.712x | 19.742x |
| corpus_count | 0.990x | 0.982x | 1.087x |
| corpus_materialize | 0.999x | 0.998x | 1.054x |
| corpus_uci_sorted | 1.009x | 1.004x | 1.570x |
| replay_trusted_native | 1.039x | 1.003x | 0.919x |
| replay_checked_native | 1.093x | 1.069x | 1.146x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).

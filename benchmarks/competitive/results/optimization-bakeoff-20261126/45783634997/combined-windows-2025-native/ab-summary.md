| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.991x | 1.291x |
| materialize | 1.025x | 0.999x | 1.200x |
| perft_bulk_d4 | 1.003x | 0.979x | 1.344x |
| perft_full_d3 | 1.003x | 0.925x | 1.245x |
| fen_parse | 1.122x | 1.028x | 1.294x |
| fen_write_native_ep | 1.606x | 1.496x | 1.565x |
| snapshot | 1.032x | 0.998x | 0.859x |
| san_parse_all | 1.063x | 1.014x | 2.064x |
| san_render_native | 1.027x | 1.000x | 3.714x |
| apply_children_full | 1.032x | 0.991x | 1.196x |
| make_unmake_full | 1.014x | 0.981x | 1.248x |
| startpos | 0.853x | 0.853x | 1.008x |
| chess960_materialize | 0.953x | 0.900x | 1.183x |
| chess960_perft_d3 | 0.973x | 0.938x | 1.299x |
| polyglot_key | 0.986x | 0.966x | 27.440x |
| polyglot_recompute | 0.981x | 0.973x | 1.036x |
| repetition_key | 1.054x | 1.039x | 19.406x |
| corpus_count | 1.006x | 0.990x | 1.476x |
| corpus_materialize | 0.985x | 0.961x | 1.157x |
| corpus_uci_sorted | 0.982x | 0.974x | 1.396x |
| replay_trusted_native | 0.969x | 0.955x | 0.653x |
| replay_checked_native | 0.988x | 0.979x | 0.946x |

Cold first legal move list: 0.034 ms -> 0.034 ms (0.98x).

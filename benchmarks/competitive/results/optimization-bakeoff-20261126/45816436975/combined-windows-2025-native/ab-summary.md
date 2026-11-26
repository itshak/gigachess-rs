| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.959x | 0.777x | 1.303x |
| materialize | 0.958x | 0.812x | 1.344x |
| perft_bulk_d4 | 0.854x | 0.765x | 1.017x |
| perft_full_d3 | 0.958x | 0.904x | 1.536x |
| fen_parse | 0.901x | 0.877x | 1.098x |
| fen_write_native_ep | 0.983x | 0.964x | 1.646x |
| snapshot | 1.712x | 1.633x | 1.116x |
| san_parse_all | 1.009x | 0.986x | 1.756x |
| san_render_native | 1.035x | 0.990x | 4.272x |
| apply_children_full | 1.059x | 0.985x | 1.196x |
| make_unmake_full | 1.023x | 0.920x | 1.125x |
| startpos | 1.370x | 1.370x | 1.146x |
| chess960_materialize | 0.936x | 0.903x | 1.257x |
| chess960_perft_d3 | 0.976x | 0.973x | 1.345x |
| polyglot_key | 1.588x | 1.401x | 42.745x |
| polyglot_recompute | 1.023x | 0.981x | 1.084x |
| repetition_key | 1.368x | 1.098x | 26.523x |
| corpus_count | 0.661x | 0.511x | 0.824x |
| corpus_materialize | 0.961x | 0.940x | 1.040x |
| corpus_uci_sorted | 0.980x | 0.972x | 1.600x |
| replay_trusted_native | 0.977x | 0.971x | 0.768x |
| replay_checked_native | 0.985x | 0.951x | 1.084x |

Cold first legal move list: 0.035 ms -> 0.018 ms (1.95x).

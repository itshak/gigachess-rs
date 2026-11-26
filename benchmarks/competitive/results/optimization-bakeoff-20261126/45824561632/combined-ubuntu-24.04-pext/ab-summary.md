| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.967x | 0.912x | 1.382x |
| materialize | 1.072x | 1.040x | 1.498x |
| perft_bulk_d4 | 0.980x | 0.954x | 1.032x |
| perft_full_d3 | 0.881x | 0.833x | 1.302x |
| fen_parse | 1.015x | 0.947x | 1.119x |
| fen_write_native_ep | 0.989x | 0.981x | 1.882x |
| snapshot | 0.612x | 0.607x | 0.933x |
| san_parse_all | 0.945x | 0.919x | 1.811x |
| san_render_native | 0.989x | 0.961x | 2.114x |
| apply_children_full | 0.985x | 0.957x | 1.303x |
| make_unmake_full | 1.043x | 0.975x | 1.200x |
| startpos | 0.925x | 0.925x | 1.300x |
| chess960_materialize | 0.878x | 0.801x | 1.270x |
| chess960_perft_d3 | 0.948x | 0.904x | 1.308x |
| polyglot_key | 1.066x | 0.997x | 39.323x |
| polyglot_recompute | 0.962x | 0.959x | 0.979x |
| repetition_key | 1.216x | 1.043x | 21.205x |
| corpus_count | 0.990x | 0.982x | 0.979x |
| corpus_materialize | 0.991x | 0.988x | 1.037x |
| corpus_uci_sorted | 1.007x | 1.006x | 1.541x |
| replay_trusted_native | 0.955x | 0.944x | 0.834x |
| replay_checked_native | 0.908x | 0.900x | 0.964x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.03x).

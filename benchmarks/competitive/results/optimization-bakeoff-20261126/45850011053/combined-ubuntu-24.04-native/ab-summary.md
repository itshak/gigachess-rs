| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.971x | 1.493x |
| materialize | 0.942x | 0.898x | 1.522x |
| perft_bulk_d4 | 0.962x | 0.940x | 1.130x |
| perft_full_d3 | 0.958x | 0.943x | 1.303x |
| fen_parse | 0.919x | 0.887x | 1.021x |
| fen_write_native_ep | 0.996x | 0.982x | 2.052x |
| snapshot | 0.995x | 0.989x | 1.150x |
| san_parse_all | 1.053x | 1.013x | 1.824x |
| san_render_native | 0.955x | 0.898x | 1.992x |
| apply_children_full | 0.994x | 0.965x | 1.294x |
| make_unmake_full | 0.874x | 0.839x | 1.028x |
| startpos | 0.833x | 0.833x | 0.842x |
| chess960_materialize | 0.954x | 0.886x | 1.435x |
| chess960_perft_d3 | 0.928x | 0.904x | 1.388x |
| polyglot_key | 1.005x | 1.003x | 40.022x |
| polyglot_recompute | 0.958x | 0.955x | 0.986x |
| repetition_key | 0.800x | 0.683x | 19.238x |
| corpus_count | 0.849x | 0.732x | 0.945x |
| corpus_materialize | 0.995x | 0.990x | 1.064x |
| corpus_uci_sorted | 0.993x | 0.993x | 1.533x |
| replay_trusted_native | 0.939x | 0.921x | 0.812x |
| replay_checked_native | 0.940x | 0.926x | 0.989x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).

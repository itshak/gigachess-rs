| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.937x | 0.889x | 1.297x |
| materialize | 0.948x | 0.931x | 1.062x |
| perft_bulk_d4 | 0.924x | 0.877x | 1.223x |
| perft_full_d3 | 0.918x | 0.881x | 1.028x |
| fen_parse | 1.074x | 1.043x | 1.387x |
| fen_write_native_ep | 0.929x | 0.873x | 1.493x |
| snapshot | 0.667x | 0.660x | 1.000x |
| san_parse_all | 0.967x | 0.948x | 1.862x |
| san_render_native | 0.975x | 0.937x | 3.266x |
| apply_children_full | 0.891x | 0.826x | 1.174x |
| make_unmake_full | 0.885x | 0.811x | 1.129x |
| startpos | 1.000x | 1.000x | 2.757x |
| chess960_materialize | 0.959x | 0.955x | 1.220x |
| chess960_perft_d3 | 0.943x | 0.943x | 1.379x |
| polyglot_key | 1.027x | 1.013x | 20.901x |
| polyglot_recompute | 1.027x | 1.011x | 0.966x |
| repetition_key | 1.014x | 0.957x | 19.355x |
| corpus_count | 0.974x | 0.965x | 1.134x |
| corpus_materialize | 0.989x | 0.984x | 1.105x |
| corpus_uci_sorted | 1.049x | 1.021x | 1.455x |
| replay_trusted_native | 0.914x | 0.854x | 0.922x |
| replay_checked_native | 1.008x | 1.006x | 0.993x |

Cold first legal move list: 0.017 ms -> 0.022 ms (0.78x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.927x | 0.881x | 1.443x |
| materialize | 0.978x | 0.941x | 1.301x |
| perft_bulk_d4 | 0.863x | 0.842x | 1.204x |
| perft_full_d3 | 0.758x | 0.683x | 1.033x |
| fen_parse | 0.984x | 0.950x | 1.200x |
| fen_write_native_ep | 0.945x | 0.908x | 1.505x |
| snapshot | 1.557x | 1.553x | 1.231x |
| san_parse_all | 0.945x | 0.929x | 1.735x |
| san_render_native | 0.815x | 0.750x | 2.166x |
| apply_children_full | 0.872x | 0.768x | 1.002x |
| make_unmake_full | 0.803x | 0.756x | 1.247x |
| startpos | 1.045x | 1.045x | 1.237x |
| chess960_materialize | 0.965x | 0.955x | 1.443x |
| chess960_perft_d3 | 0.888x | 0.821x | 1.333x |
| polyglot_key | 1.043x | 0.950x | 26.202x |
| polyglot_recompute | 1.003x | 1.002x | 1.097x |
| repetition_key | 1.147x | 1.062x | 21.690x |
| corpus_count | 0.940x | 0.921x | 1.257x |
| corpus_materialize | 0.983x | 0.979x | 1.208x |
| corpus_uci_sorted | 1.028x | 1.028x | 1.499x |
| replay_trusted_native | 0.847x | 0.800x | 0.531x |
| replay_checked_native | 0.878x | 0.873x | 0.993x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.95x).

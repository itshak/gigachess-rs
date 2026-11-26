| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.928x | 0.848x | 1.416x |
| materialize | 0.971x | 0.925x | 1.179x |
| perft_bulk_d4 | 0.946x | 0.884x | 1.306x |
| perft_full_d3 | 0.847x | 0.679x | 0.915x |
| fen_parse | 1.061x | 0.919x | 1.222x |
| fen_write_native_ep | 1.000x | 0.968x | 1.605x |
| snapshot | 0.993x | 0.985x | 1.026x |
| san_parse_all | 0.922x | 0.894x | 1.954x |
| san_render_native | 0.862x | 0.826x | 3.205x |
| apply_children_full | 0.919x | 0.817x | 1.255x |
| make_unmake_full | 0.876x | 0.830x | 1.234x |
| startpos | 0.938x | 0.938x | 1.170x |
| chess960_materialize | 0.833x | 0.668x | 1.052x |
| chess960_perft_d3 | 0.893x | 0.824x | 1.536x |
| polyglot_key | 0.945x | 0.899x | 26.896x |
| polyglot_recompute | 0.925x | 0.855x | 0.926x |
| repetition_key | 1.028x | 0.950x | 21.237x |
| corpus_count | 0.964x | 0.951x | 1.343x |
| corpus_materialize | 0.995x | 0.983x | 1.154x |
| corpus_uci_sorted | 0.985x | 0.977x | 1.433x |
| replay_trusted_native | 0.919x | 0.887x | 0.779x |
| replay_checked_native | 0.857x | 0.790x | 0.895x |

Cold first legal move list: 0.021 ms -> 0.017 ms (1.24x).

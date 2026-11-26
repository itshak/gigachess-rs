| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.193x | 1.154x | 1.292x |
| materialize | 1.031x | 1.000x | 1.212x |
| perft_bulk_d4 | 1.444x | 1.405x | 1.290x |
| perft_full_d3 | 1.734x | 1.594x | 1.050x |
| fen_parse | 1.032x | 1.018x | 1.229x |
| fen_write_native_ep | 0.980x | 0.953x | 1.039x |
| snapshot | 1.306x | 1.302x | 0.905x |
| san_parse_all | 2.028x | 1.937x | 1.849x |
| san_render_native | 2.149x | 1.693x | 2.455x |
| apply_children_full | 1.228x | 1.147x | 0.803x |
| make_unmake_full | 1.343x | 1.246x | 1.196x |
| startpos | 1.457x | 1.457x | 1.055x |
| chess960_materialize | 1.174x | 1.049x | 1.296x |
| chess960_perft_d3 | 1.521x | 1.351x | 1.271x |
| polyglot_key | 1.063x | 0.987x | 24.625x |
| polyglot_recompute | 0.906x | 0.904x | 1.028x |
| repetition_key | 1.361x | 1.205x | 18.697x |
| corpus_count | 1.176x | 1.137x | 1.449x |
| corpus_materialize | 1.063x | 1.058x | 1.227x |
| corpus_uci_sorted | 1.651x | 1.649x | 0.935x |
| replay_trusted_native | 1.386x | 1.343x | 0.447x |
| replay_checked_native | 1.339x | 1.303x | 0.887x |

Cold first legal move list: 154.043 ms -> 0.012 ms (12601.69x).

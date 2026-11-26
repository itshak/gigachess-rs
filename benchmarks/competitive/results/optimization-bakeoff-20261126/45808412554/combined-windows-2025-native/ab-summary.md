| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.151x | 0.883x | 1.595x |
| materialize | 1.038x | 0.908x | 1.288x |
| perft_bulk_d4 | 1.020x | 0.817x | 1.368x |
| perft_full_d3 | 1.018x | 0.934x | 1.507x |
| fen_parse | 0.968x | 0.879x | 1.341x |
| fen_write_native_ep | 0.950x | 0.855x | 1.611x |
| snapshot | 0.996x | 0.817x | 1.046x |
| san_parse_all | 0.988x | 0.935x | 1.872x |
| san_render_native | 0.979x | 0.921x | 4.462x |
| apply_children_full | 0.980x | 0.921x | 1.197x |
| make_unmake_full | 0.964x | 0.864x | 1.236x |
| startpos | 2.304x | 2.304x | 0.990x |
| chess960_materialize | 1.065x | 1.031x | 1.216x |
| chess960_perft_d3 | 1.145x | 1.061x | 1.428x |
| polyglot_key | 1.003x | 0.998x | 36.709x |
| polyglot_recompute | 1.015x | 0.932x | 1.071x |
| repetition_key | 0.999x | 0.961x | 15.644x |
| corpus_count | 1.136x | 1.058x | 1.507x |
| corpus_materialize | 1.009x | 0.990x | 1.124x |
| corpus_uci_sorted | 1.022x | 1.004x | 1.416x |
| replay_trusted_native | 1.029x | 0.967x | 0.949x |
| replay_checked_native | 1.034x | 1.019x | 1.035x |

Cold first legal move list: 0.033 ms -> 0.029 ms (1.15x).

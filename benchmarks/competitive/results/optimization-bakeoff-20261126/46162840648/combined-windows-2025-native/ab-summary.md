| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 1.024x | 1.552x |
| materialize | 0.979x | 0.950x | 1.207x |
| perft_bulk_d4 | 1.055x | 0.974x | 1.409x |
| perft_full_d3 | 1.214x | 1.040x | 1.606x |
| fen_parse | 1.002x | 0.910x | 1.382x |
| fen_write_native_ep | 1.021x | 0.993x | 1.615x |
| snapshot | 0.989x | 0.969x | 0.993x |
| san_parse_all | 0.994x | 0.961x | 2.107x |
| san_render_native | 1.006x | 0.903x | 3.813x |
| apply_children_full | 1.026x | 0.966x | 1.442x |
| make_unmake_full | 1.006x | 0.952x | 1.362x |
| startpos | 1.001x | 1.001x | 1.254x |
| chess960_materialize | 0.979x | 0.971x | 1.309x |
| chess960_perft_d3 | 1.002x | 0.993x | 1.551x |
| polyglot_key | 0.952x | 0.935x | 26.151x |
| polyglot_recompute | 1.291x | 1.184x | 1.304x |
| repetition_key | 0.983x | 0.972x | 20.896x |
| corpus_count | 1.068x | 1.064x | 1.477x |
| corpus_materialize | 0.979x | 0.955x | 1.128x |
| corpus_uci_sorted | 0.991x | 0.975x | 1.389x |
| replay_trusted_native | 0.975x | 0.927x | 0.825x |
| replay_checked_native | 0.924x | 0.899x | 1.041x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.09x).

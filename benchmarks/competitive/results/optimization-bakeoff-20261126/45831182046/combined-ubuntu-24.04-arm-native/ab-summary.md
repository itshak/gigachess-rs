| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.037x | 0.989x | 1.428x |
| materialize | 1.011x | 0.987x | 1.271x |
| perft_bulk_d4 | 1.038x | 0.989x | 1.292x |
| perft_full_d3 | 1.008x | 0.994x | 1.059x |
| fen_parse | 1.009x | 1.006x | 1.221x |
| fen_write_native_ep | 0.998x | 0.996x | 1.599x |
| snapshot | 1.021x | 1.017x | 0.937x |
| san_parse_all | 0.993x | 0.987x | 1.822x |
| san_render_native | 1.054x | 1.023x | 2.681x |
| apply_children_full | 1.001x | 0.985x | 1.155x |
| make_unmake_full | 1.006x | 0.989x | 1.551x |
| startpos | 1.001x | 1.001x | 1.275x |
| chess960_materialize | 1.051x | 0.987x | 1.458x |
| chess960_perft_d3 | 1.065x | 1.015x | 1.426x |
| polyglot_key | 1.021x | 1.005x | 25.871x |
| polyglot_recompute | 1.005x | 1.001x | 1.089x |
| repetition_key | 1.028x | 1.003x | 19.392x |
| corpus_count | 1.043x | 1.033x | 1.246x |
| corpus_materialize | 1.013x | 1.010x | 1.187x |
| corpus_uci_sorted | 1.005x | 1.005x | 1.452x |
| replay_trusted_native | 1.028x | 1.023x | 0.617x |
| replay_checked_native | 1.089x | 1.070x | 1.142x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.27x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.026x | 0.989x | 1.548x |
| materialize | 1.014x | 0.992x | 1.207x |
| perft_bulk_d4 | 0.935x | 0.884x | 1.272x |
| perft_full_d3 | 0.979x | 0.955x | 0.895x |
| fen_parse | 0.865x | 0.804x | 1.083x |
| fen_write_native_ep | 1.030x | 1.003x | 1.898x |
| snapshot | 1.015x | 0.980x | 1.246x |
| san_parse_all | 1.012x | 0.993x | 2.297x |
| san_render_native | 0.832x | 0.803x | 1.764x |
| apply_children_full | 0.861x | 0.789x | 1.099x |
| make_unmake_full | 0.878x | 0.826x | 1.086x |
| startpos | 1.009x | 1.009x | 1.197x |
| chess960_materialize | 0.952x | 0.923x | 1.505x |
| chess960_perft_d3 | 0.942x | 0.905x | 1.522x |
| polyglot_key | 0.985x | 0.983x | 20.329x |
| polyglot_recompute | 0.977x | 0.957x | 1.008x |
| repetition_key | 0.997x | 0.979x | 18.847x |
| corpus_count | 1.004x | 0.982x | 1.360x |
| corpus_materialize | 0.982x | 0.961x | 1.254x |
| corpus_uci_sorted | 1.001x | 0.994x | 1.665x |
| replay_trusted_native | 0.814x | 0.777x | 0.890x |
| replay_checked_native | 1.003x | 1.001x | 1.020x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.00x).

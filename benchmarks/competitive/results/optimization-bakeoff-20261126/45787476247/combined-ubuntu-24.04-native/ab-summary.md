| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.982x | 1.278x |
| materialize | 0.970x | 0.929x | 1.223x |
| perft_bulk_d4 | 0.985x | 0.959x | 1.273x |
| perft_full_d3 | 0.980x | 0.953x | 1.218x |
| fen_parse | 1.015x | 0.982x | 1.196x |
| fen_write_native_ep | 0.989x | 0.878x | 1.703x |
| snapshot | 0.996x | 0.956x | 1.034x |
| san_parse_all | 0.970x | 0.954x | 1.944x |
| san_render_native | 1.003x | 0.982x | 2.362x |
| apply_children_full | 0.946x | 0.859x | 1.087x |
| make_unmake_full | 0.974x | 0.948x | 1.167x |
| startpos | 1.001x | 1.001x | 1.199x |
| chess960_materialize | 0.994x | 0.981x | 1.270x |
| chess960_perft_d3 | 0.991x | 0.982x | 1.312x |
| polyglot_key | 0.930x | 0.921x | 24.437x |
| polyglot_recompute | 1.025x | 1.021x | 1.065x |
| repetition_key | 0.982x | 0.955x | 20.014x |
| corpus_count | 1.007x | 0.994x | 1.344x |
| corpus_materialize | 1.005x | 0.990x | 1.190x |
| corpus_uci_sorted | 0.984x | 0.971x | 1.714x |
| replay_trusted_native | 0.981x | 0.972x | 0.701x |
| replay_checked_native | 1.002x | 0.977x | 1.029x |

Cold first legal move list: 0.014 ms -> 0.014 ms (0.96x).

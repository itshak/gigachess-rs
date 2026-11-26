| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.869x | 0.754x | 1.352x |
| materialize | 0.924x | 0.852x | 1.172x |
| perft_bulk_d4 | 0.911x | 0.809x | 1.278x |
| perft_full_d3 | 0.902x | 0.766x | 1.024x |
| fen_parse | 1.096x | 1.065x | 1.295x |
| fen_write_native_ep | 0.899x | 0.734x | 1.529x |
| snapshot | 0.930x | 0.808x | 0.972x |
| san_parse_all | 0.932x | 0.859x | 2.100x |
| san_render_native | 0.936x | 0.878x | 3.754x |
| apply_children_full | 0.936x | 0.862x | 1.322x |
| make_unmake_full | 0.916x | 0.878x | 1.353x |
| startpos | 0.990x | 0.990x | 1.257x |
| chess960_materialize | 0.981x | 0.968x | 1.271x |
| chess960_perft_d3 | 0.984x | 0.977x | 1.550x |
| polyglot_key | 0.988x | 0.984x | 29.550x |
| polyglot_recompute | 1.017x | 1.015x | 1.002x |
| repetition_key | 1.003x | 0.999x | 22.809x |
| corpus_count | 1.002x | 0.995x | 1.422x |
| corpus_materialize | 0.965x | 0.950x | 1.121x |
| corpus_uci_sorted | 0.983x | 0.979x | 1.441x |
| replay_trusted_native | 1.001x | 0.985x | 0.848x |
| replay_checked_native | 1.038x | 1.018x | 1.091x |

Cold first legal move list: 0.024 ms -> 0.023 ms (1.04x).

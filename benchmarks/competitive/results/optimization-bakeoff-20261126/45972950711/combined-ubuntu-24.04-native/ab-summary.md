| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 1.001x | 1.518x |
| materialize | 1.064x | 1.032x | 1.587x |
| perft_bulk_d4 | 1.006x | 1.000x | 1.172x |
| perft_full_d3 | 1.020x | 1.005x | 1.361x |
| fen_parse | 0.982x | 0.968x | 1.061x |
| fen_write_native_ep | 1.003x | 0.968x | 2.120x |
| snapshot | 1.010x | 0.988x | 0.999x |
| san_parse_all | 0.899x | 0.892x | 1.684x |
| san_render_native | 0.975x | 0.959x | 2.105x |
| apply_children_full | 1.013x | 0.991x | 1.367x |
| make_unmake_full | 0.998x | 0.952x | 1.187x |
| startpos | 0.799x | 0.799x | 1.001x |
| chess960_materialize | 0.985x | 0.981x | 1.580x |
| chess960_perft_d3 | 0.989x | 0.988x | 1.500x |
| polyglot_key | 0.928x | 0.866x | 37.692x |
| polyglot_recompute | 0.963x | 0.956x | 1.032x |
| repetition_key | 0.962x | 0.950x | 19.171x |
| corpus_count | 1.035x | 1.021x | 1.128x |
| corpus_materialize | 1.010x | 1.010x | 1.060x |
| corpus_uci_sorted | 1.006x | 1.005x | 1.571x |
| replay_trusted_native | 1.004x | 0.979x | 0.895x |
| replay_checked_native | 0.949x | 0.944x | 1.133x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).

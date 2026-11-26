| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.962x | 0.823x | 1.268x |
| materialize | 0.981x | 0.873x | 1.175x |
| perft_bulk_d4 | 0.920x | 0.823x | 1.183x |
| perft_full_d3 | 0.999x | 0.944x | 0.861x |
| fen_parse | 0.920x | 0.783x | 1.169x |
| fen_write_native_ep | 0.966x | 0.935x | 1.506x |
| snapshot | 0.746x | 0.723x | 0.746x |
| san_parse_all | 0.961x | 0.857x | 1.935x |
| san_render_native | 0.957x | 0.851x | 3.739x |
| apply_children_full | 1.126x | 1.052x | 1.171x |
| make_unmake_full | 0.964x | 0.922x | 1.231x |
| startpos | 1.004x | 1.004x | 1.253x |
| chess960_materialize | 1.051x | 1.012x | 1.224x |
| chess960_perft_d3 | 1.013x | 0.958x | 1.368x |
| polyglot_key | 0.993x | 0.971x | 37.169x |
| polyglot_recompute | 0.988x | 0.979x | 0.987x |
| repetition_key | 1.535x | 1.405x | 24.588x |
| corpus_count | 0.867x | 0.751x | 1.082x |
| corpus_materialize | 0.937x | 0.921x | 1.074x |
| corpus_uci_sorted | 0.989x | 0.966x | 1.361x |
| replay_trusted_native | 0.988x | 0.970x | 0.708x |
| replay_checked_native | 1.014x | 1.002x | 0.932x |

Cold first legal move list: 0.021 ms -> 0.013 ms (1.62x).

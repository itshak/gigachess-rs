| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.959x | 1.185x |
| materialize | 0.957x | 0.882x | 1.276x |
| perft_bulk_d4 | 0.971x | 0.919x | 1.161x |
| perft_full_d3 | 0.985x | 0.957x | 1.286x |
| fen_parse | 1.112x | 1.071x | 1.232x |
| fen_write_native_ep | 0.923x | 0.864x | 1.362x |
| snapshot | 1.030x | 1.007x | 0.998x |
| san_parse_all | 1.011x | 0.987x | 1.739x |
| san_render_native | 0.934x | 0.749x | 2.112x |
| apply_children_full | 0.992x | 0.836x | 1.248x |
| make_unmake_full | 0.982x | 0.937x | 1.236x |
| startpos | 1.154x | 1.154x | 1.502x |
| chess960_materialize | 0.910x | 0.907x | 1.462x |
| chess960_perft_d3 | 0.935x | 0.910x | 1.391x |
| polyglot_key | 1.036x | 1.031x | 32.160x |
| polyglot_recompute | 0.988x | 0.979x | 0.977x |
| repetition_key | 1.057x | 1.037x | 22.332x |
| corpus_count | 0.970x | 0.967x | 1.024x |
| corpus_materialize | 1.000x | 0.998x | 1.182x |
| corpus_uci_sorted | 0.993x | 0.992x | 1.609x |
| replay_trusted_native | 0.948x | 0.947x | 0.697x |
| replay_checked_native | 0.942x | 0.916x | 1.202x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.82x).

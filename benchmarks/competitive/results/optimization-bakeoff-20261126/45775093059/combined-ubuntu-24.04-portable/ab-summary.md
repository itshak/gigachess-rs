| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.042x | 0.995x | 1.147x |
| materialize | 1.009x | 0.975x | 1.548x |
| perft_bulk_d4 | 1.201x | 1.122x | 1.143x |
| perft_full_d3 | 2.304x | 2.043x | 1.459x |
| fen_parse | 1.065x | 1.022x | 1.175x |
| fen_write_native_ep | 0.977x | 0.966x | 1.143x |
| snapshot | 2.236x | 2.209x | 0.998x |
| san_parse_all | 2.338x | 2.175x | 1.820x |
| san_render_native | 2.670x | 2.375x | 2.223x |
| apply_children_full | 2.323x | 2.188x | 0.977x |
| make_unmake_full | 1.359x | 1.254x | 1.031x |
| startpos | 1.278x | 1.278x | 1.280x |
| chess960_materialize | 1.259x | 1.138x | 1.414x |
| chess960_perft_d3 | 1.289x | 1.184x | 1.300x |
| polyglot_key | 1.197x | 1.102x | 39.976x |
| polyglot_recompute | 0.924x | 0.899x | 0.892x |
| repetition_key | 1.456x | 1.308x | 23.671x |
| corpus_count | 1.051x | 1.044x | 1.247x |
| corpus_materialize | 1.013x | 1.013x | 1.171x |
| corpus_uci_sorted | 1.541x | 1.538x | 0.935x |
| replay_trusted_native | 1.301x | 1.236x | 0.641x |
| replay_checked_native | 1.266x | 1.151x | 0.985x |

Cold first legal move list: 123.148 ms -> 0.010 ms (12847.98x).

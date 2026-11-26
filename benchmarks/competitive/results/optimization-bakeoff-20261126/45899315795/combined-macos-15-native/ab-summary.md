| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.864x | 0.808x | 1.368x |
| materialize | 1.100x | 0.876x | 1.061x |
| perft_bulk_d4 | 0.998x | 0.907x | 1.230x |
| perft_full_d3 | 0.879x | 0.776x | 1.150x |
| fen_parse | 0.987x | 0.815x | 1.183x |
| fen_write_native_ep | 0.929x | 0.804x | 1.679x |
| snapshot | 0.984x | 0.592x | 0.910x |
| san_parse_all | 1.119x | 0.988x | 1.826x |
| san_render_native | 1.048x | 0.906x | 2.839x |
| apply_children_full | 1.044x | 0.896x | 1.191x |
| make_unmake_full | 0.899x | 0.658x | 1.188x |
| startpos | 1.009x | 1.009x | 1.205x |
| chess960_materialize | 1.035x | 1.025x | 1.482x |
| chess960_perft_d3 | 1.009x | 0.987x | 1.530x |
| polyglot_key | 1.018x | 0.869x | 38.942x |
| polyglot_recompute | 0.902x | 0.804x | 0.926x |
| repetition_key | 1.046x | 0.994x | 20.662x |
| corpus_count | 0.975x | 0.936x | 1.102x |
| corpus_materialize | 0.980x | 0.915x | 1.185x |
| corpus_uci_sorted | 1.055x | 0.936x | 1.478x |
| replay_trusted_native | 0.959x | 0.900x | 0.677x |
| replay_checked_native | 0.943x | 0.925x | 1.153x |

Cold first legal move list: 0.003 ms -> 0.003 ms (1.13x).

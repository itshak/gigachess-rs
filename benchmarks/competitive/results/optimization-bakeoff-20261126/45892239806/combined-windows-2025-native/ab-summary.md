| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.925x | 0.880x | 1.361x |
| materialize | 0.933x | 0.881x | 1.134x |
| perft_bulk_d4 | 0.915x | 0.822x | 1.175x |
| perft_full_d3 | 0.932x | 0.900x | 1.174x |
| fen_parse | 0.947x | 0.889x | 1.297x |
| fen_write_native_ep | 0.923x | 0.845x | 1.393x |
| snapshot | 0.920x | 0.893x | 1.377x |
| san_parse_all | 1.002x | 0.962x | 1.926x |
| san_render_native | 0.778x | 0.600x | 3.185x |
| apply_children_full | 0.615x | 0.495x | 0.861x |
| make_unmake_full | 0.942x | 0.917x | 1.234x |
| startpos | 1.065x | 1.065x | 0.913x |
| chess960_materialize | 0.854x | 0.818x | 1.203x |
| chess960_perft_d3 | 0.920x | 0.919x | 1.417x |
| polyglot_key | 1.000x | 0.915x | 34.975x |
| polyglot_recompute | 1.029x | 0.983x | 0.962x |
| repetition_key | 0.982x | 0.892x | 26.910x |
| corpus_count | 0.963x | 0.928x | 1.090x |
| corpus_materialize | 1.045x | 1.019x | 1.102x |
| corpus_uci_sorted | 1.109x | 1.097x | 1.488x |
| replay_trusted_native | 1.120x | 1.052x | 0.801x |
| replay_checked_native | 0.947x | 0.944x | 1.121x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.89x).

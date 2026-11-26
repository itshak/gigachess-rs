| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.925x | 0.898x | 1.182x |
| materialize | 0.931x | 0.905x | 1.612x |
| perft_bulk_d4 | 0.939x | 0.888x | 1.114x |
| perft_full_d3 | 0.933x | 0.917x | 1.236x |
| fen_parse | 1.000x | 0.945x | 1.051x |
| fen_write_native_ep | 0.916x | 0.852x | 1.747x |
| snapshot | 1.010x | 0.997x | 1.119x |
| san_parse_all | 0.976x | 0.952x | 1.852x |
| san_render_native | 0.975x | 0.924x | 2.164x |
| apply_children_full | 0.983x | 0.945x | 1.336x |
| make_unmake_full | 0.973x | 0.900x | 1.162x |
| startpos | 1.393x | 1.393x | 1.896x |
| chess960_materialize | 0.850x | 0.816x | 1.323x |
| chess960_perft_d3 | 0.913x | 0.892x | 1.377x |
| polyglot_key | 1.000x | 0.988x | 43.949x |
| polyglot_recompute | 0.998x | 0.978x | 1.025x |
| repetition_key | 0.941x | 0.927x | 20.325x |
| corpus_count | 0.970x | 0.951x | 1.094x |
| corpus_materialize | 0.970x | 0.965x | 1.117x |
| corpus_uci_sorted | 1.004x | 1.002x | 1.570x |
| replay_trusted_native | 0.952x | 0.949x | 0.829x |
| replay_checked_native | 0.904x | 0.880x | 1.068x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).

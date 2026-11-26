| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.910x | 1.374x |
| materialize | 0.993x | 0.965x | 1.212x |
| perft_bulk_d4 | 0.932x | 0.885x | 1.212x |
| perft_full_d3 | 1.005x | 0.968x | 1.261x |
| fen_parse | 1.034x | 0.898x | 1.308x |
| fen_write_native_ep | 0.984x | 0.905x | 1.688x |
| snapshot | 1.289x | 1.217x | 0.975x |
| san_parse_all | 0.995x | 0.947x | 2.101x |
| san_render_native | 0.983x | 0.895x | 3.776x |
| apply_children_full | 1.118x | 1.039x | 1.353x |
| make_unmake_full | 1.007x | 0.970x | 1.355x |
| startpos | 1.166x | 1.166x | 1.253x |
| chess960_materialize | 0.959x | 0.924x | 1.244x |
| chess960_perft_d3 | 1.006x | 0.901x | 1.351x |
| polyglot_key | 0.830x | 0.796x | 26.131x |
| polyglot_recompute | 0.999x | 0.994x | 1.063x |
| repetition_key | 0.995x | 0.967x | 19.829x |
| corpus_count | 0.871x | 0.835x | 1.221x |
| corpus_materialize | 0.904x | 0.865x | 1.057x |
| corpus_uci_sorted | 0.913x | 0.881x | 1.303x |
| replay_trusted_native | 1.119x | 1.077x | 0.841x |
| replay_checked_native | 1.005x | 0.968x | 1.030x |

Cold first legal move list: 0.035 ms -> 0.017 ms (2.07x).

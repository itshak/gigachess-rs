| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.888x | 0.648x | 1.434x |
| materialize | 1.002x | 0.927x | 1.174x |
| perft_bulk_d4 | 0.945x | 0.791x | 1.155x |
| perft_full_d3 | 1.003x | 0.871x | 1.310x |
| fen_parse | 1.051x | 0.895x | 1.263x |
| fen_write_native_ep | 0.962x | 0.780x | 1.699x |
| snapshot | 0.923x | 0.638x | 0.883x |
| san_parse_all | 0.969x | 0.806x | 1.805x |
| san_render_native | 1.053x | 0.953x | 2.731x |
| apply_children_full | 0.881x | 0.667x | 1.019x |
| make_unmake_full | 0.853x | 0.716x | 1.151x |
| startpos | 1.004x | 1.004x | 1.212x |
| chess960_materialize | 0.988x | 0.905x | 1.458x |
| chess960_perft_d3 | 0.957x | 0.881x | 1.714x |
| polyglot_key | 1.081x | 0.949x | 40.470x |
| polyglot_recompute | 1.134x | 1.061x | 1.099x |
| repetition_key | 1.075x | 1.069x | 21.503x |
| corpus_count | 0.833x | 0.623x | 1.031x |
| corpus_materialize | 1.055x | 0.855x | 1.153x |
| corpus_uci_sorted | 1.150x | 1.092x | 1.289x |
| replay_trusted_native | 1.018x | 0.977x | 0.615x |
| replay_checked_native | 1.181x | 1.178x | 1.146x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.94x).

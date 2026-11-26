| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.058x | 0.989x | 1.369x |
| materialize | 1.025x | 0.844x | 1.004x |
| perft_bulk_d4 | 1.106x | 0.923x | 1.339x |
| perft_full_d3 | 0.994x | 0.898x | 1.331x |
| fen_parse | 0.972x | 0.876x | 1.190x |
| fen_write_native_ep | 1.052x | 0.972x | 1.807x |
| snapshot | 1.035x | 0.599x | 0.903x |
| san_parse_all | 1.076x | 0.966x | 1.881x |
| san_render_native | 0.991x | 0.898x | 2.616x |
| apply_children_full | 0.921x | 0.806x | 0.892x |
| make_unmake_full | 1.017x | 0.819x | 1.183x |
| startpos | 0.868x | 0.868x | 1.169x |
| chess960_materialize | 0.999x | 0.976x | 1.404x |
| chess960_perft_d3 | 0.937x | 0.915x | 1.425x |
| polyglot_key | 1.046x | 1.001x | 37.933x |
| polyglot_recompute | 1.045x | 1.037x | 0.922x |
| repetition_key | 1.073x | 0.930x | 21.083x |
| corpus_count | 1.013x | 0.881x | 1.296x |
| corpus_materialize | 0.990x | 0.961x | 1.177x |
| corpus_uci_sorted | 1.075x | 0.957x | 1.454x |
| replay_trusted_native | 0.855x | 0.674x | 0.503x |
| replay_checked_native | 1.065x | 0.921x | 1.091x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.12x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.999x | 1.427x |
| materialize | 0.999x | 0.993x | 1.565x |
| perft_bulk_d4 | 0.973x | 0.905x | 1.034x |
| perft_full_d3 | 0.987x | 0.955x | 1.455x |
| fen_parse | 1.066x | 1.000x | 1.176x |
| fen_write_native_ep | 0.957x | 0.942x | 1.988x |
| snapshot | 1.004x | 0.993x | 0.999x |
| san_parse_all | 1.003x | 0.993x | 1.900x |
| san_render_native | 0.974x | 0.957x | 2.093x |
| apply_children_full | 1.003x | 0.995x | 1.199x |
| make_unmake_full | 1.000x | 0.985x | 1.116x |
| startpos | 0.811x | 0.811x | 1.003x |
| chess960_materialize | 0.983x | 0.960x | 1.333x |
| chess960_perft_d3 | 1.010x | 1.006x | 1.383x |
| polyglot_key | 1.037x | 1.010x | 40.692x |
| polyglot_recompute | 0.999x | 0.995x | 1.052x |
| repetition_key | 1.024x | 1.023x | 17.273x |
| corpus_count | 1.047x | 1.021x | 0.967x |
| corpus_materialize | 1.013x | 1.010x | 1.088x |
| corpus_uci_sorted | 0.995x | 0.992x | 1.522x |
| replay_trusted_native | 0.984x | 0.978x | 0.810x |
| replay_checked_native | 1.011x | 1.009x | 1.075x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).

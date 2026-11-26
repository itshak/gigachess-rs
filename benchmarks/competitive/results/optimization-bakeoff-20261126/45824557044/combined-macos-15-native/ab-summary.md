| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.906x | 1.321x |
| materialize | 0.993x | 0.907x | 1.026x |
| perft_bulk_d4 | 0.903x | 0.759x | 0.983x |
| perft_full_d3 | 1.024x | 0.970x | 1.422x |
| fen_parse | 0.988x | 0.911x | 1.221x |
| fen_write_native_ep | 0.975x | 0.913x | 1.766x |
| snapshot | 0.943x | 0.757x | 0.995x |
| san_parse_all | 1.062x | 0.974x | 1.803x |
| san_render_native | 1.010x | 0.926x | 2.531x |
| apply_children_full | 1.008x | 0.864x | 1.076x |
| make_unmake_full | 1.065x | 1.019x | 1.243x |
| startpos | 1.067x | 1.067x | 1.198x |
| chess960_materialize | 1.011x | 0.999x | 1.458x |
| chess960_perft_d3 | 1.000x | 0.993x | 1.230x |
| polyglot_key | 1.027x | 1.016x | 37.738x |
| polyglot_recompute | 1.065x | 1.060x | 0.975x |
| repetition_key | 1.045x | 0.981x | 18.920x |
| corpus_count | 1.032x | 1.007x | 0.989x |
| corpus_materialize | 1.035x | 1.026x | 1.124x |
| corpus_uci_sorted | 1.073x | 1.033x | 1.358x |
| replay_trusted_native | 1.079x | 1.052x | 0.642x |
| replay_checked_native | 1.020x | 0.994x | 1.152x |

Cold first legal move list: 0.004 ms -> 0.007 ms (0.57x).

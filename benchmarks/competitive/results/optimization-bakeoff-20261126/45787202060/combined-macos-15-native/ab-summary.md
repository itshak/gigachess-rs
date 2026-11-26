| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.881x | 0.731x | 1.262x |
| materialize | 1.007x | 0.775x | 1.028x |
| perft_bulk_d4 | 0.940x | 0.529x | 0.959x |
| perft_full_d3 | 0.964x | 0.812x | 1.169x |
| fen_parse | 0.915x | 0.770x | 1.077x |
| fen_write_native_ep | 1.107x | 0.881x | 1.730x |
| snapshot | 1.422x | 0.784x | 1.141x |
| san_parse_all | 1.054x | 0.850x | 1.770x |
| san_render_native | 1.129x | 0.801x | 2.609x |
| apply_children_full | 0.955x | 0.766x | 0.825x |
| make_unmake_full | 0.597x | 0.438x | 0.644x |
| startpos | 1.292x | 1.292x | 1.313x |
| chess960_materialize | 1.200x | 1.162x | 1.231x |
| chess960_perft_d3 | 0.820x | 0.760x | 1.096x |
| polyglot_key | 1.147x | 0.767x | 41.520x |
| polyglot_recompute | 1.116x | 1.031x | 0.972x |
| repetition_key | 1.004x | 0.919x | 25.004x |
| corpus_count | 0.772x | 0.766x | 1.422x |
| corpus_materialize | 0.882x | 0.843x | 1.001x |
| corpus_uci_sorted | 0.780x | 0.663x | 1.258x |
| replay_trusted_native | 0.914x | 0.650x | 0.551x |
| replay_checked_native | 0.876x | 0.814x | 0.878x |

Cold first legal move list: 0.018 ms -> 0.014 ms (1.28x).

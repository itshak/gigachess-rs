| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.944x | 0.712x | 1.535x |
| materialize | 0.917x | 0.786x | 1.033x |
| perft_bulk_d4 | 0.993x | 0.903x | 1.309x |
| perft_full_d3 | 0.961x | 0.689x | 1.124x |
| fen_parse | 0.996x | 0.821x | 1.242x |
| fen_write_native_ep | 0.932x | 0.729x | 1.745x |
| snapshot | 0.801x | 0.495x | 0.801x |
| san_parse_all | 0.921x | 0.759x | 1.780x |
| san_render_native | 1.004x | 0.866x | 2.755x |
| apply_children_full | 0.995x | 0.820x | 1.123x |
| make_unmake_full | 0.928x | 0.777x | 1.260x |
| startpos | 0.903x | 0.903x | 0.939x |
| chess960_materialize | 1.422x | 1.236x | 1.364x |
| chess960_perft_d3 | 1.274x | 1.197x | 1.507x |
| polyglot_key | 1.116x | 1.065x | 36.472x |
| polyglot_recompute | 1.300x | 1.217x | 1.026x |
| repetition_key | 1.518x | 1.475x | 21.368x |
| corpus_count | 1.081x | 0.979x | 1.212x |
| corpus_materialize | 1.252x | 1.147x | 1.093x |
| corpus_uci_sorted | 1.184x | 1.086x | 1.530x |
| replay_trusted_native | 1.134x | 0.964x | 0.651x |
| replay_checked_native | 1.180x | 0.993x | 1.233x |

Cold first legal move list: 0.007 ms -> 0.003 ms (2.69x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.981x | 1.570x |
| materialize | 1.014x | 1.000x | 1.342x |
| perft_bulk_d4 | 1.045x | 1.019x | 1.460x |
| perft_full_d3 | 1.133x | 1.007x | 1.510x |
| fen_parse | 0.990x | 0.985x | 1.207x |
| fen_write_native_ep | 0.999x | 0.997x | 1.572x |
| snapshot | 1.000x | 0.996x | 0.946x |
| san_parse_all | 0.999x | 0.994x | 1.832x |
| san_render_native | 1.001x | 0.993x | 2.690x |
| apply_children_full | 1.001x | 0.968x | 1.158x |
| make_unmake_full | 1.017x | 1.004x | 1.587x |
| startpos | 1.001x | 1.001x | 1.304x |
| chess960_materialize | 1.002x | 1.002x | 1.503x |
| chess960_perft_d3 | 1.019x | 1.003x | 1.518x |
| polyglot_key | 0.989x | 0.987x | 25.780x |
| polyglot_recompute | 1.003x | 0.998x | 1.084x |
| repetition_key | 0.994x | 0.987x | 19.295x |
| corpus_count | 1.092x | 1.060x | 1.501x |
| corpus_materialize | 1.010x | 1.009x | 1.248x |
| corpus_uci_sorted | 0.998x | 0.998x | 1.457x |
| replay_trusted_native | 1.003x | 0.996x | 0.636x |
| replay_checked_native | 1.011x | 1.010x | 1.137x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.07x).

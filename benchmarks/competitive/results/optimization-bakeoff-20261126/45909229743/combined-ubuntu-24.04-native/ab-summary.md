| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.058x | 1.012x | 1.576x |
| materialize | 1.083x | 1.049x | 1.574x |
| perft_bulk_d4 | 1.062x | 1.050x | 1.251x |
| perft_full_d3 | 1.236x | 1.057x | 1.682x |
| fen_parse | 0.924x | 0.826x | 0.990x |
| fen_write_native_ep | 0.995x | 0.980x | 2.151x |
| snapshot | 0.997x | 0.993x | 1.153x |
| san_parse_all | 1.011x | 0.991x | 1.898x |
| san_render_native | 0.999x | 0.997x | 2.124x |
| apply_children_full | 1.000x | 0.997x | 1.338x |
| make_unmake_full | 1.000x | 0.982x | 1.203x |
| startpos | 0.999x | 0.999x | 1.128x |
| chess960_materialize | 1.066x | 1.051x | 1.521x |
| chess960_perft_d3 | 1.010x | 0.991x | 1.522x |
| polyglot_key | 1.002x | 0.998x | 39.777x |
| polyglot_recompute | 0.963x | 0.956x | 0.959x |
| repetition_key | 1.184x | 0.997x | 23.478x |
| corpus_count | 1.230x | 1.136x | 1.343x |
| corpus_materialize | 1.008x | 1.004x | 1.054x |
| corpus_uci_sorted | 1.000x | 0.994x | 1.554x |
| replay_trusted_native | 1.057x | 1.008x | 0.925x |
| replay_checked_native | 1.019x | 1.010x | 1.234x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.91x).

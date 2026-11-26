| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.959x | 0.906x | 1.224x |
| materialize | 0.920x | 0.898x | 1.271x |
| perft_bulk_d4 | 0.905x | 0.884x | 1.184x |
| perft_full_d3 | 0.947x | 0.926x | 1.212x |
| fen_parse | 1.113x | 1.073x | 1.277x |
| fen_write_native_ep | 0.952x | 0.924x | 1.562x |
| snapshot | 1.202x | 1.197x | 1.175x |
| san_parse_all | 0.939x | 0.889x | 1.938x |
| san_render_native | 0.985x | 0.934x | 2.181x |
| apply_children_full | 0.942x | 0.870x | 1.289x |
| make_unmake_full | 0.919x | 0.876x | 1.372x |
| startpos | 1.597x | 1.597x | 2.111x |
| chess960_materialize | 0.910x | 0.903x | 1.345x |
| chess960_perft_d3 | 0.949x | 0.943x | 1.436x |
| polyglot_key | 1.051x | 1.003x | 29.482x |
| polyglot_recompute | 1.001x | 0.992x | 1.034x |
| repetition_key | 1.065x | 1.053x | 21.473x |
| corpus_count | 0.972x | 0.969x | 1.236x |
| corpus_materialize | 0.970x | 0.966x | 1.169x |
| corpus_uci_sorted | 1.036x | 1.032x | 1.779x |
| replay_trusted_native | 0.817x | 0.760x | 0.728x |
| replay_checked_native | 0.864x | 0.863x | 1.002x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.84x).

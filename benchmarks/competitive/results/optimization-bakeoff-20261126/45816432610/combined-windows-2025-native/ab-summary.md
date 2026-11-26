| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.954x | 1.357x |
| materialize | 0.991x | 0.930x | 1.097x |
| perft_bulk_d4 | 1.017x | 0.912x | 1.222x |
| perft_full_d3 | 1.240x | 1.203x | 1.242x |
| fen_parse | 0.990x | 0.965x | 1.409x |
| fen_write_native_ep | 1.006x | 0.981x | 1.517x |
| snapshot | 1.224x | 1.179x | 1.495x |
| san_parse_all | 1.008x | 0.934x | 1.936x |
| san_render_native | 1.047x | 0.966x | 3.744x |
| apply_children_full | 1.169x | 1.074x | 1.207x |
| make_unmake_full | 0.983x | 0.955x | 1.242x |
| startpos | 1.029x | 1.029x | 1.260x |
| chess960_materialize | 1.179x | 1.131x | 1.301x |
| chess960_perft_d3 | 1.079x | 0.955x | 1.397x |
| polyglot_key | 1.098x | 1.059x | 37.595x |
| polyglot_recompute | 0.994x | 0.982x | 0.962x |
| repetition_key | 1.633x | 1.530x | 23.226x |
| corpus_count | 0.887x | 0.780x | 1.090x |
| corpus_materialize | 0.934x | 0.868x | 1.043x |
| corpus_uci_sorted | 1.085x | 1.065x | 1.488x |
| replay_trusted_native | 0.926x | 0.823x | 0.595x |
| replay_checked_native | 1.037x | 1.024x | 0.985x |

Cold first legal move list: 0.019 ms -> 0.014 ms (1.41x).

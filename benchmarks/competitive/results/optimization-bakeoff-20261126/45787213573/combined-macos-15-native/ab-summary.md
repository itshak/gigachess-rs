| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.043x | 0.900x | 1.358x |
| materialize | 1.024x | 0.925x | 1.004x |
| perft_bulk_d4 | 1.024x | 0.965x | 1.320x |
| perft_full_d3 | 1.015x | 0.935x | 1.320x |
| fen_parse | 1.094x | 1.017x | 1.240x |
| fen_write_native_ep | 0.996x | 0.902x | 1.815x |
| snapshot | 1.051x | 0.914x | 1.042x |
| san_parse_all | 0.986x | 0.811x | 1.913x |
| san_render_native | 1.000x | 0.904x | 2.596x |
| apply_children_full | 1.034x | 0.797x | 0.930x |
| make_unmake_full | 0.998x | 0.909x | 1.188x |
| startpos | 1.012x | 1.012x | 1.591x |
| chess960_materialize | 0.970x | 0.914x | 1.374x |
| chess960_perft_d3 | 0.901x | 0.897x | 1.358x |
| polyglot_key | 0.949x | 0.856x | 35.995x |
| polyglot_recompute | 1.086x | 1.081x | 0.930x |
| repetition_key | 1.011x | 0.962x | 19.874x |
| corpus_count | 1.023x | 0.884x | 1.437x |
| corpus_materialize | 0.821x | 0.807x | 0.997x |
| corpus_uci_sorted | 0.889x | 0.840x | 1.544x |
| replay_trusted_native | 0.808x | 0.643x | 0.443x |
| replay_checked_native | 1.169x | 1.131x | 1.168x |

Cold first legal move list: 0.014 ms -> 0.011 ms (1.26x).

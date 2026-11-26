| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.079x | 0.814x | 1.265x |
| materialize | 1.063x | 0.911x | 1.007x |
| perft_bulk_d4 | 1.167x | 1.021x | 1.290x |
| perft_full_d3 | 1.128x | 0.887x | 1.297x |
| fen_parse | 1.320x | 1.177x | 1.550x |
| fen_write_native_ep | 1.022x | 0.777x | 1.734x |
| snapshot | 1.108x | 0.842x | 0.956x |
| san_parse_all | 1.020x | 0.817x | 1.744x |
| san_render_native | 1.057x | 0.994x | 2.574x |
| apply_children_full | 1.056x | 0.849x | 0.955x |
| make_unmake_full | 1.241x | 0.969x | 1.231x |
| startpos | 1.016x | 1.016x | 1.604x |
| chess960_materialize | 1.099x | 1.078x | 1.367x |
| chess960_perft_d3 | 0.986x | 0.933x | 1.438x |
| polyglot_key | 1.084x | 1.037x | 37.485x |
| polyglot_recompute | 1.177x | 1.067x | 1.034x |
| repetition_key | 1.233x | 1.175x | 19.865x |
| corpus_count | 1.092x | 1.058x | 1.330x |
| corpus_materialize | 1.021x | 1.014x | 1.075x |
| corpus_uci_sorted | 0.928x | 0.740x | 1.430x |
| replay_trusted_native | 0.867x | 0.841x | 0.567x |
| replay_checked_native | 1.133x | 1.111x | 0.906x |

Cold first legal move list: 0.017 ms -> 0.016 ms (1.05x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.043x | 0.780x | 1.616x |
| materialize | 0.942x | 0.836x | 1.138x |
| perft_bulk_d4 | 0.941x | 0.838x | 1.268x |
| perft_full_d3 | 0.982x | 0.775x | 1.350x |
| fen_parse | 0.980x | 0.727x | 1.127x |
| fen_write_native_ep | 1.062x | 0.800x | 1.743x |
| snapshot | 0.976x | 0.720x | 0.979x |
| san_parse_all | 0.979x | 0.784x | 1.768x |
| san_render_native | 1.009x | 0.799x | 2.829x |
| apply_children_full | 1.030x | 0.798x | 1.092x |
| make_unmake_full | 1.011x | 0.862x | 1.231x |
| startpos | 1.444x | 1.444x | 1.103x |
| chess960_materialize | 0.933x | 0.867x | 1.500x |
| chess960_perft_d3 | 0.873x | 0.848x | 1.470x |
| polyglot_key | 1.007x | 0.876x | 38.881x |
| polyglot_recompute | 1.100x | 0.968x | 1.111x |
| repetition_key | 0.923x | 0.899x | 21.838x |
| corpus_count | 0.979x | 0.916x | 1.175x |
| corpus_materialize | 1.011x | 0.897x | 1.150x |
| corpus_uci_sorted | 1.011x | 0.929x | 1.484x |
| replay_trusted_native | 0.915x | 0.889x | 0.645x |
| replay_checked_native | 1.048x | 0.871x | 1.302x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.16x).

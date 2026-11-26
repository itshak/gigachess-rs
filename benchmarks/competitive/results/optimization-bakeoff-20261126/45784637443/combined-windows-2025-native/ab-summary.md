| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.084x | 1.001x | 1.381x |
| materialize | 1.039x | 0.969x | 1.177x |
| perft_bulk_d4 | 1.115x | 1.004x | 1.286x |
| perft_full_d3 | 1.089x | 0.956x | 1.043x |
| fen_parse | 1.168x | 1.056x | 1.390x |
| fen_write_native_ep | 1.050x | 1.009x | 1.522x |
| snapshot | 1.207x | 1.172x | 1.386x |
| san_parse_all | 1.044x | 1.013x | 1.980x |
| san_render_native | 1.093x | 1.007x | 3.813x |
| apply_children_full | 1.209x | 1.170x | 1.212x |
| make_unmake_full | 1.030x | 0.990x | 1.320x |
| startpos | 1.059x | 1.059x | 1.253x |
| chess960_materialize | 1.179x | 1.079x | 1.327x |
| chess960_perft_d3 | 1.119x | 1.056x | 1.459x |
| polyglot_key | 1.007x | 0.987x | 35.049x |
| polyglot_recompute | 1.047x | 1.043x | 0.958x |
| repetition_key | 1.082x | 1.048x | 25.212x |
| corpus_count | 1.058x | 1.002x | 1.211x |
| corpus_materialize | 1.085x | 1.074x | 1.144x |
| corpus_uci_sorted | 1.096x | 1.032x | 1.364x |
| replay_trusted_native | 1.075x | 1.035x | 0.719x |
| replay_checked_native | 1.074x | 1.034x | 0.950x |

Cold first legal move list: 0.020 ms -> 0.016 ms (1.26x).

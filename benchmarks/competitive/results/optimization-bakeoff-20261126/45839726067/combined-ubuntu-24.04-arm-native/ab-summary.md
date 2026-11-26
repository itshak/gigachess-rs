| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.001x | 1.570x |
| materialize | 1.004x | 0.999x | 1.326x |
| perft_bulk_d4 | 1.002x | 0.993x | 1.400x |
| perft_full_d3 | 0.992x | 0.976x | 0.825x |
| fen_parse | 1.216x | 1.149x | 1.466x |
| fen_write_native_ep | 0.998x | 0.993x | 1.586x |
| snapshot | 0.989x | 0.985x | 0.945x |
| san_parse_all | 0.998x | 0.996x | 1.830x |
| san_render_native | 1.073x | 1.021x | 2.706x |
| apply_children_full | 0.999x | 0.979x | 1.154x |
| make_unmake_full | 0.999x | 0.979x | 1.551x |
| startpos | 1.001x | 1.001x | 1.302x |
| chess960_materialize | 1.002x | 1.001x | 1.496x |
| chess960_perft_d3 | 1.000x | 0.995x | 1.498x |
| polyglot_key | 1.002x | 1.001x | 25.663x |
| polyglot_recompute | 1.043x | 1.031x | 1.138x |
| repetition_key | 0.996x | 0.992x | 19.433x |
| corpus_count | 1.000x | 0.996x | 1.385x |
| corpus_materialize | 0.999x | 0.995x | 1.220x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.456x |
| replay_trusted_native | 0.999x | 0.989x | 0.610x |
| replay_checked_native | 1.065x | 1.041x | 1.118x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.06x).

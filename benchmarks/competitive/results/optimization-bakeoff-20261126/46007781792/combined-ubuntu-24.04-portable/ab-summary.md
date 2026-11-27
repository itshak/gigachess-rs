| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.078x | 1.034x | 1.411x |
| materialize | 1.048x | 1.001x | 1.324x |
| perft_bulk_d4 | 1.062x | 1.034x | 1.342x |
| perft_full_d3 | 1.234x | 1.015x | 1.586x |
| fen_parse | 0.858x | 0.811x | 1.068x |
| fen_write_native_ep | 1.030x | 1.011x | 1.686x |
| snapshot | 1.000x | 0.999x | 1.127x |
| san_parse_all | 1.034x | 1.019x | 1.964x |
| san_render_native | 1.030x | 0.996x | 2.284x |
| apply_children_full | 0.997x | 0.971x | 1.410x |
| make_unmake_full | 1.019x | 0.948x | 1.389x |
| startpos | 0.998x | 0.998x | 1.329x |
| chess960_materialize | 1.001x | 1.001x | 1.500x |
| chess960_perft_d3 | 1.040x | 1.023x | 1.584x |
| polyglot_key | 1.001x | 1.000x | 22.038x |
| polyglot_recompute | 1.002x | 1.000x | 0.990x |
| repetition_key | 1.007x | 1.003x | 21.025x |
| corpus_count | 1.148x | 1.125x | 1.327x |
| corpus_materialize | 1.027x | 1.009x | 1.319x |
| corpus_uci_sorted | 1.038x | 1.037x | 1.819x |
| replay_trusted_native | 1.036x | 0.999x | 1.073x |
| replay_checked_native | 1.070x | 1.040x | 1.176x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.09x).

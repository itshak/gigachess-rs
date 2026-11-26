| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.979x | 1.358x |
| materialize | 1.003x | 0.986x | 1.361x |
| perft_bulk_d4 | 0.999x | 0.980x | 1.262x |
| perft_full_d3 | 0.975x | 0.959x | 0.849x |
| fen_parse | 1.141x | 1.119x | 1.188x |
| fen_write_native_ep | 0.997x | 0.986x | 1.345x |
| snapshot | 1.415x | 1.410x | 1.063x |
| san_parse_all | 1.005x | 0.997x | 1.793x |
| san_render_native | 1.000x | 0.985x | 2.387x |
| apply_children_full | 1.149x | 1.106x | 0.966x |
| make_unmake_full | 1.031x | 0.998x | 1.031x |
| startpos | 0.814x | 0.814x | 1.080x |
| chess960_materialize | 0.989x | 0.987x | 1.302x |
| chess960_perft_d3 | 1.002x | 0.997x | 1.231x |
| polyglot_key | 1.010x | 1.006x | 30.563x |
| polyglot_recompute | 1.005x | 1.004x | 1.114x |
| repetition_key | 1.007x | 1.004x | 20.103x |
| corpus_count | 1.003x | 1.003x | 1.347x |
| corpus_materialize | 0.992x | 0.988x | 1.180x |
| corpus_uci_sorted | 1.004x | 1.000x | 0.971x |
| replay_trusted_native | 1.078x | 1.070x | 0.613x |
| replay_checked_native | 1.019x | 0.995x | 0.972x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.91x).

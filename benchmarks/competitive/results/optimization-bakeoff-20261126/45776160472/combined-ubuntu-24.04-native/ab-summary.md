| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.987x | 1.242x |
| materialize | 1.004x | 0.995x | 1.310x |
| perft_bulk_d4 | 0.999x | 0.990x | 1.182x |
| perft_full_d3 | 0.970x | 0.963x | 0.842x |
| fen_parse | 0.921x | 0.879x | 1.122x |
| fen_write_native_ep | 1.407x | 1.265x | 1.896x |
| snapshot | 1.115x | 1.106x | 1.135x |
| san_parse_all | 0.981x | 0.974x | 1.831x |
| san_render_native | 1.012x | 1.002x | 2.394x |
| apply_children_full | 0.948x | 0.930x | 0.823x |
| make_unmake_full | 0.975x | 0.923x | 0.991x |
| startpos | 1.096x | 1.096x | 1.119x |
| chess960_materialize | 1.005x | 1.002x | 1.320x |
| chess960_perft_d3 | 1.015x | 1.012x | 1.209x |
| polyglot_key | 1.055x | 0.998x | 32.623x |
| polyglot_recompute | 0.955x | 0.945x | 1.067x |
| repetition_key | 0.976x | 0.951x | 19.642x |
| corpus_count | 1.000x | 0.999x | 1.203x |
| corpus_materialize | 0.997x | 0.996x | 1.119x |
| corpus_uci_sorted | 0.999x | 0.992x | 0.976x |
| replay_trusted_native | 0.977x | 0.964x | 0.775x |
| replay_checked_native | 1.016x | 1.011x | 0.899x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.93x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.554x |
| materialize | 0.931x | 0.886x | 1.561x |
| perft_bulk_d4 | 1.021x | 1.005x | 1.206x |
| perft_full_d3 | 1.005x | 1.000x | 1.364x |
| fen_parse | 0.944x | 0.849x | 1.053x |
| fen_write_native_ep | 0.964x | 0.916x | 2.064x |
| snapshot | 0.998x | 0.991x | 1.328x |
| san_parse_all | 1.101x | 1.078x | 1.894x |
| san_render_native | 1.016x | 1.008x | 2.145x |
| apply_children_full | 0.990x | 0.965x | 1.214x |
| make_unmake_full | 0.915x | 0.870x | 1.070x |
| startpos | 1.114x | 1.114x | 1.247x |
| chess960_materialize | 1.019x | 1.018x | 1.514x |
| chess960_perft_d3 | 0.997x | 0.993x | 1.472x |
| polyglot_key | 1.013x | 0.998x | 39.663x |
| polyglot_recompute | 1.056x | 1.041x | 1.025x |
| repetition_key | 0.821x | 0.700x | 19.473x |
| corpus_count | 0.995x | 0.990x | 1.067x |
| corpus_materialize | 1.002x | 0.999x | 1.096x |
| corpus_uci_sorted | 1.002x | 1.001x | 1.583x |
| replay_trusted_native | 1.007x | 0.969x | 0.925x |
| replay_checked_native | 1.015x | 1.003x | 1.059x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.05x).

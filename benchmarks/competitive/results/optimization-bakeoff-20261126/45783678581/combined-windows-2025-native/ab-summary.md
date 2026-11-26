| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.974x | 1.326x |
| materialize | 0.989x | 0.943x | 1.154x |
| perft_bulk_d4 | 0.961x | 0.936x | 1.253x |
| perft_full_d3 | 0.898x | 0.802x | 0.988x |
| fen_parse | 0.992x | 0.954x | 1.382x |
| fen_write_native_ep | 1.004x | 0.961x | 1.298x |
| snapshot | 0.831x | 0.808x | 1.001x |
| san_parse_all | 0.974x | 0.917x | 1.979x |
| san_render_native | 0.933x | 0.879x | 3.680x |
| apply_children_full | 0.823x | 0.727x | 1.105x |
| make_unmake_full | 0.988x | 0.913x | 1.278x |
| startpos | 0.996x | 0.996x | 1.264x |
| chess960_materialize | 0.996x | 0.990x | 1.166x |
| chess960_perft_d3 | 0.987x | 0.968x | 1.347x |
| polyglot_key | 0.965x | 0.964x | 36.806x |
| polyglot_recompute | 0.935x | 0.895x | 0.973x |
| repetition_key | 0.976x | 0.915x | 24.823x |
| corpus_count | 0.996x | 0.948x | 1.270x |
| corpus_materialize | 0.937x | 0.852x | 1.147x |
| corpus_uci_sorted | 0.948x | 0.890x | 1.292x |
| replay_trusted_native | 1.034x | 0.978x | 0.669x |
| replay_checked_native | 1.220x | 1.117x | 1.104x |

Cold first legal move list: 0.023 ms -> 0.016 ms (1.40x).

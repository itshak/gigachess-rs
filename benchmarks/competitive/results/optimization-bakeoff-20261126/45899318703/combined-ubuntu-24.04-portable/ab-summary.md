| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.987x | 1.278x |
| materialize | 1.014x | 0.987x | 1.375x |
| perft_bulk_d4 | 1.004x | 0.991x | 1.275x |
| perft_full_d3 | 1.012x | 0.984x | 1.283x |
| fen_parse | 1.001x | 0.970x | 1.061x |
| fen_write_native_ep | 0.993x | 0.970x | 1.722x |
| snapshot | 1.010x | 1.000x | 0.986x |
| san_parse_all | 1.004x | 0.998x | 2.175x |
| san_render_native | 1.013x | 0.979x | 2.292x |
| apply_children_full | 1.015x | 1.005x | 1.421x |
| make_unmake_full | 1.007x | 0.992x | 1.381x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 1.037x | 1.025x | 1.658x |
| chess960_perft_d3 | 1.011x | 0.999x | 1.566x |
| polyglot_key | 1.020x | 1.019x | 21.832x |
| polyglot_recompute | 1.019x | 1.016x | 0.976x |
| repetition_key | 1.007x | 1.006x | 20.200x |
| corpus_count | 1.010x | 1.004x | 1.227x |
| corpus_materialize | 1.030x | 1.011x | 1.348x |
| corpus_uci_sorted | 1.012x | 1.005x | 1.594x |
| replay_trusted_native | 0.987x | 0.983x | 1.066x |
| replay_checked_native | 1.015x | 1.013x | 1.158x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.09x).

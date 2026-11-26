| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 1.008x | 1.526x |
| materialize | 1.000x | 0.987x | 1.166x |
| perft_bulk_d4 | 0.969x | 0.933x | 1.304x |
| perft_full_d3 | 1.179x | 1.066x | 1.284x |
| fen_parse | 1.019x | 0.995x | 1.331x |
| fen_write_native_ep | 1.019x | 1.005x | 1.531x |
| snapshot | 0.999x | 0.966x | 1.490x |
| san_parse_all | 1.000x | 0.990x | 1.935x |
| san_render_native | 1.004x | 0.992x | 3.816x |
| apply_children_full | 0.996x | 0.979x | 1.334x |
| make_unmake_full | 1.006x | 0.981x | 1.302x |
| startpos | 1.024x | 1.024x | 1.002x |
| chess960_materialize | 0.996x | 0.995x | 1.391x |
| chess960_perft_d3 | 1.010x | 1.004x | 1.490x |
| polyglot_key | 0.990x | 0.956x | 36.127x |
| polyglot_recompute | 1.020x | 1.013x | 0.978x |
| repetition_key | 0.991x | 0.983x | 27.928x |
| corpus_count | 1.016x | 1.004x | 1.155x |
| corpus_materialize | 1.015x | 1.008x | 1.063x |
| corpus_uci_sorted | 1.003x | 0.997x | 1.386x |
| replay_trusted_native | 1.010x | 1.008x | 0.688x |
| replay_checked_native | 1.000x | 1.000x | 1.158x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.92x).

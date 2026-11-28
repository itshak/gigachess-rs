| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.080x | 1.030x | 1.355x |
| materialize | 0.975x | 0.901x | 1.289x |
| perft_bulk_d4 | 1.082x | 1.055x | 1.308x |
| perft_full_d3 | 1.246x | 1.034x | 1.687x |
| fen_parse | 0.898x | 0.886x | 1.174x |
| fen_write_native_ep | 1.000x | 0.981x | 1.535x |
| snapshot | 1.008x | 0.972x | 0.999x |
| san_parse_all | 1.011x | 1.000x | 1.737x |
| san_render_native | 1.005x | 0.992x | 2.330x |
| apply_children_full | 0.998x | 0.962x | 1.261x |
| make_unmake_full | 1.024x | 1.001x | 1.320x |
| startpos | 1.010x | 1.010x | 1.293x |
| chess960_materialize | 1.020x | 1.010x | 1.587x |
| chess960_perft_d3 | 1.028x | 1.013x | 1.534x |
| polyglot_key | 0.990x | 0.971x | 32.017x |
| polyglot_recompute | 1.302x | 1.044x | 1.319x |
| repetition_key | 1.009x | 1.002x | 25.294x |
| corpus_count | 1.123x | 1.119x | 1.209x |
| corpus_materialize | 1.008x | 1.007x | 1.206x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.589x |
| replay_trusted_native | 1.009x | 1.005x | 0.736x |
| replay_checked_native | 1.014x | 0.978x | 1.281x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.85x).

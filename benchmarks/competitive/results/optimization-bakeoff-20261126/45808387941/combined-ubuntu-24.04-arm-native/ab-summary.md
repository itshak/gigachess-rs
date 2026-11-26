| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.934x | 1.371x |
| materialize | 1.006x | 0.966x | 1.248x |
| perft_bulk_d4 | 0.914x | 0.899x | 1.233x |
| perft_full_d3 | 0.982x | 0.964x | 1.044x |
| fen_parse | 1.006x | 0.999x | 1.250x |
| fen_write_native_ep | 0.999x | 0.997x | 1.590x |
| snapshot | 0.914x | 0.912x | 0.949x |
| san_parse_all | 1.001x | 0.997x | 1.826x |
| san_render_native | 0.999x | 0.979x | 2.545x |
| apply_children_full | 1.068x | 1.044x | 1.195x |
| make_unmake_full | 1.032x | 1.007x | 1.483x |
| startpos | 1.076x | 1.076x | 1.292x |
| chess960_materialize | 1.044x | 0.988x | 1.390x |
| chess960_perft_d3 | 1.032x | 0.979x | 1.361x |
| polyglot_key | 1.020x | 0.989x | 25.654x |
| polyglot_recompute | 1.043x | 1.030x | 1.120x |
| repetition_key | 0.983x | 0.931x | 18.404x |
| corpus_count | 0.817x | 0.791x | 1.187x |
| corpus_materialize | 0.976x | 0.968x | 1.172x |
| corpus_uci_sorted | 1.000x | 1.000x | 1.451x |
| replay_trusted_native | 1.056x | 1.035x | 0.545x |
| replay_checked_native | 1.026x | 1.024x | 1.012x |

Cold first legal move list: 0.011 ms -> 0.007 ms (1.58x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.960x | 1.120x |
| materialize | 1.009x | 0.974x | 1.244x |
| perft_bulk_d4 | 1.003x | 0.996x | 1.170x |
| perft_full_d3 | 0.972x | 0.933x | 0.849x |
| fen_parse | 0.941x | 0.925x | 1.137x |
| fen_write_native_ep | 1.002x | 0.992x | 1.070x |
| snapshot | 1.001x | 0.999x | 1.019x |
| san_parse_all | 1.003x | 0.986x | 1.887x |
| san_render_native | 0.997x | 0.988x | 2.380x |
| apply_children_full | 0.984x | 0.959x | 0.914x |
| make_unmake_full | 0.996x | 0.990x | 1.066x |
| startpos | 1.000x | 1.000x | 1.257x |
| chess960_materialize | 1.014x | 1.002x | 1.280x |
| chess960_perft_d3 | 0.995x | 0.985x | 1.289x |
| polyglot_key | 0.989x | 0.975x | 27.801x |
| polyglot_recompute | 0.998x | 0.995x | 1.044x |
| repetition_key | 0.993x | 0.987x | 20.632x |
| corpus_count | 0.998x | 0.995x | 1.220x |
| corpus_materialize | 0.988x | 0.988x | 1.188x |
| corpus_uci_sorted | 1.011x | 1.000x | 0.970x |
| replay_trusted_native | 1.004x | 1.000x | 0.600x |
| replay_checked_native | 0.884x | 0.845x | 0.877x |

Cold first legal move list: 0.015 ms -> 0.012 ms (1.20x).

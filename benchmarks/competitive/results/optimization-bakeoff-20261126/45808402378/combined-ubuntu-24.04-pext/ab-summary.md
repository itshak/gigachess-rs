| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.916x | 1.424x |
| materialize | 0.976x | 0.933x | 1.195x |
| perft_bulk_d4 | 0.975x | 0.946x | 1.284x |
| perft_full_d3 | 1.112x | 1.056x | 0.971x |
| fen_parse | 1.014x | 1.004x | 1.195x |
| fen_write_native_ep | 1.003x | 0.995x | 1.845x |
| snapshot | 1.000x | 0.997x | 1.250x |
| san_parse_all | 1.021x | 1.009x | 2.242x |
| san_render_native | 1.000x | 0.944x | 2.127x |
| apply_children_full | 1.102x | 1.077x | 1.165x |
| make_unmake_full | 0.991x | 0.958x | 1.224x |
| startpos | 1.001x | 1.001x | 1.198x |
| chess960_materialize | 1.019x | 0.990x | 1.475x |
| chess960_perft_d3 | 0.997x | 0.969x | 1.483x |
| polyglot_key | 0.995x | 0.990x | 20.634x |
| polyglot_recompute | 0.963x | 0.960x | 0.982x |
| repetition_key | 0.815x | 0.784x | 15.734x |
| corpus_count | 0.858x | 0.771x | 1.262x |
| corpus_materialize | 0.960x | 0.952x | 1.206x |
| corpus_uci_sorted | 0.978x | 0.971x | 1.615x |
| replay_trusted_native | 0.993x | 0.976x | 0.800x |
| replay_checked_native | 0.989x | 0.988x | 0.981x |

Cold first legal move list: 0.011 ms -> 0.006 ms (1.92x).

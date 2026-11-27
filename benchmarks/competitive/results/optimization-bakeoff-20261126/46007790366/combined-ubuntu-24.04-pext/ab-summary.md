| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.076x | 1.030x | 1.550x |
| materialize | 0.997x | 0.971x | 1.259x |
| perft_bulk_d4 | 1.062x | 1.032x | 1.378x |
| perft_full_d3 | 1.558x | 1.090x | 1.640x |
| fen_parse | 0.898x | 0.838x | 1.109x |
| fen_write_native_ep | 1.010x | 0.988x | 1.543x |
| snapshot | 1.023x | 0.995x | 1.491x |
| san_parse_all | 1.030x | 1.002x | 1.964x |
| san_render_native | 1.032x | 1.016x | 2.298x |
| apply_children_full | 1.019x | 0.984x | 1.359x |
| make_unmake_full | 1.020x | 0.951x | 1.180x |
| startpos | 1.036x | 1.036x | 1.250x |
| chess960_materialize | 1.025x | 1.007x | 1.486x |
| chess960_perft_d3 | 1.033x | 1.003x | 1.533x |
| polyglot_key | 0.990x | 0.942x | 34.445x |
| polyglot_recompute | 1.262x | 1.070x | 1.219x |
| repetition_key | 1.023x | 1.014x | 27.709x |
| corpus_count | 1.137x | 1.079x | 1.282x |
| corpus_materialize | 1.070x | 1.055x | 1.214x |
| corpus_uci_sorted | 0.935x | 0.888x | 1.615x |
| replay_trusted_native | 1.020x | 1.006x | 0.757x |
| replay_checked_native | 1.014x | 1.014x | 1.232x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.14x).

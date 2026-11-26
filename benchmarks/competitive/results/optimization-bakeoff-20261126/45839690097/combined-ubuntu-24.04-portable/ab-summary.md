| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.987x | 1.299x |
| materialize | 1.070x | 1.044x | 1.366x |
| perft_bulk_d4 | 1.002x | 0.983x | 1.272x |
| perft_full_d3 | 1.431x | 1.284x | 1.287x |
| fen_parse | 0.944x | 0.884x | 1.251x |
| fen_write_native_ep | 0.988x | 0.964x | 1.693x |
| snapshot | 0.999x | 0.992x | 0.976x |
| san_parse_all | 0.993x | 0.983x | 2.158x |
| san_render_native | 0.980x | 0.878x | 2.242x |
| apply_children_full | 1.007x | 0.985x | 1.274x |
| make_unmake_full | 0.971x | 0.889x | 1.353x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 1.023x | 1.002x | 1.659x |
| chess960_perft_d3 | 0.990x | 0.985x | 1.576x |
| polyglot_key | 1.006x | 0.999x | 22.135x |
| polyglot_recompute | 0.981x | 0.979x | 0.986x |
| repetition_key | 0.989x | 0.975x | 20.591x |
| corpus_count | 0.998x | 0.998x | 1.225x |
| corpus_materialize | 0.999x | 0.997x | 1.376x |
| corpus_uci_sorted | 0.995x | 0.991x | 1.591x |
| replay_trusted_native | 1.055x | 1.017x | 0.979x |
| replay_checked_native | 0.999x | 0.995x | 1.070x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.02x).

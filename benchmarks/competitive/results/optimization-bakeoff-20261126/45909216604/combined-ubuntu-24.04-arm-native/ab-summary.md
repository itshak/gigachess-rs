| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 1.002x | 1.572x |
| materialize | 1.001x | 0.996x | 1.337x |
| perft_bulk_d4 | 1.040x | 1.022x | 1.445x |
| perft_full_d3 | 0.993x | 0.983x | 1.325x |
| fen_parse | 0.995x | 0.987x | 1.214x |
| fen_write_native_ep | 1.000x | 0.996x | 1.608x |
| snapshot | 1.002x | 0.993x | 0.947x |
| san_parse_all | 0.981x | 0.976x | 1.783x |
| san_render_native | 0.999x | 0.990x | 2.659x |
| apply_children_full | 0.996x | 0.975x | 1.152x |
| make_unmake_full | 1.001x | 0.995x | 1.566x |
| startpos | 1.001x | 1.001x | 1.307x |
| chess960_materialize | 1.004x | 1.003x | 1.498x |
| chess960_perft_d3 | 1.019x | 1.007x | 1.524x |
| polyglot_key | 0.987x | 0.982x | 25.427x |
| polyglot_recompute | 0.997x | 0.996x | 1.085x |
| repetition_key | 0.998x | 0.997x | 19.345x |
| corpus_count | 1.104x | 1.067x | 1.483x |
| corpus_materialize | 0.993x | 0.992x | 1.224x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.464x |
| replay_trusted_native | 0.999x | 0.997x | 0.626x |
| replay_checked_native | 0.998x | 0.992x | 1.127x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).

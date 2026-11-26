| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.960x | 0.883x | 1.488x |
| materialize | 0.988x | 0.952x | 1.321x |
| perft_bulk_d4 | 0.981x | 0.938x | 1.369x |
| perft_full_d3 | 1.145x | 1.018x | 1.523x |
| fen_parse | 0.987x | 0.982x | 1.217x |
| fen_write_native_ep | 0.999x | 0.996x | 1.595x |
| snapshot | 0.993x | 0.990x | 0.934x |
| san_parse_all | 0.997x | 0.989x | 1.826x |
| san_render_native | 1.000x | 0.987x | 2.670x |
| apply_children_full | 0.994x | 0.988x | 1.144x |
| make_unmake_full | 1.010x | 0.997x | 1.589x |
| startpos | 1.000x | 1.000x | 1.331x |
| chess960_materialize | 0.997x | 0.994x | 1.497x |
| chess960_perft_d3 | 0.988x | 0.975x | 1.483x |
| polyglot_key | 1.011x | 0.992x | 25.522x |
| polyglot_recompute | 0.993x | 0.990x | 1.083x |
| repetition_key | 0.999x | 0.998x | 18.924x |
| corpus_count | 0.916x | 0.912x | 1.271x |
| corpus_materialize | 0.972x | 0.971x | 1.229x |
| corpus_uci_sorted | 1.001x | 1.001x | 1.458x |
| replay_trusted_native | 1.007x | 1.006x | 0.627x |
| replay_checked_native | 1.008x | 1.003x | 1.138x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).

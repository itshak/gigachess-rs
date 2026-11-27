| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 0.997x | 1.347x |
| materialize | 0.999x | 0.984x | 1.756x |
| perft_bulk_d4 | 1.064x | 1.037x | 1.263x |
| perft_full_d3 | 1.170x | 1.038x | 1.547x |
| fen_parse | 0.997x | 0.937x | 1.134x |
| fen_write_native_ep | 1.012x | 1.009x | 2.034x |
| snapshot | 0.999x | 0.989x | 0.999x |
| san_parse_all | 0.930x | 0.893x | 1.775x |
| san_render_native | 1.015x | 0.998x | 2.183x |
| apply_children_full | 1.002x | 0.997x | 1.379x |
| make_unmake_full | 1.065x | 1.012x | 1.305x |
| startpos | 1.003x | 1.003x | 1.286x |
| chess960_materialize | 1.006x | 1.006x | 1.457x |
| chess960_perft_d3 | 1.013x | 0.996x | 1.519x |
| polyglot_key | 0.938x | 0.880x | 39.375x |
| polyglot_recompute | 1.007x | 0.992x | 1.008x |
| repetition_key | 0.949x | 0.904x | 20.933x |
| corpus_count | 1.182x | 1.106x | 1.350x |
| corpus_materialize | 1.002x | 0.997x | 1.139x |
| corpus_uci_sorted | 0.984x | 0.981x | 1.523x |
| replay_trusted_native | 1.008x | 0.984x | 0.877x |
| replay_checked_native | 1.009x | 1.001x | 1.145x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.01x).

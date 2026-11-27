| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.862x | 1.558x |
| materialize | 1.003x | 0.972x | 1.153x |
| perft_bulk_d4 | 1.012x | 0.998x | 1.467x |
| perft_full_d3 | 1.272x | 1.074x | 1.640x |
| fen_parse | 1.035x | 1.005x | 1.364x |
| fen_write_native_ep | 1.001x | 0.992x | 1.642x |
| snapshot | 0.974x | 0.865x | 1.158x |
| san_parse_all | 0.999x | 0.992x | 1.960x |
| san_render_native | 1.015x | 0.999x | 3.373x |
| apply_children_full | 0.988x | 0.965x | 1.314x |
| make_unmake_full | 0.975x | 0.934x | 1.282x |
| startpos | 1.002x | 1.002x | 1.002x |
| chess960_materialize | 0.999x | 0.991x | 1.352x |
| chess960_perft_d3 | 1.003x | 0.993x | 1.612x |
| polyglot_key | 1.046x | 0.998x | 21.137x |
| polyglot_recompute | 1.038x | 1.031x | 0.987x |
| repetition_key | 1.000x | 0.995x | 19.096x |
| corpus_count | 1.076x | 1.068x | 1.398x |
| corpus_materialize | 0.974x | 0.954x | 1.193x |
| corpus_uci_sorted | 1.013x | 1.013x | 1.529x |
| replay_trusted_native | 1.054x | 0.988x | 1.145x |
| replay_checked_native | 1.023x | 0.997x | 1.082x |

Cold first legal move list: 0.018 ms -> 0.016 ms (1.13x).

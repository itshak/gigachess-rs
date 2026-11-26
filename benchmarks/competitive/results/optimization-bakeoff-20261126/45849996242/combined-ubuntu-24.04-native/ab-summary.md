| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.992x | 1.453x |
| materialize | 1.015x | 0.972x | 1.222x |
| perft_bulk_d4 | 1.017x | 0.998x | 1.272x |
| perft_full_d3 | 1.040x | 0.820x | 1.276x |
| fen_parse | 1.142x | 1.096x | 1.306x |
| fen_write_native_ep | 1.009x | 0.985x | 1.539x |
| snapshot | 0.714x | 0.696x | 1.065x |
| san_parse_all | 1.000x | 0.937x | 1.711x |
| san_render_native | 0.981x | 0.939x | 2.208x |
| apply_children_full | 1.005x | 0.997x | 1.307x |
| make_unmake_full | 1.048x | 0.977x | 1.248x |
| startpos | 0.991x | 0.991x | 1.268x |
| chess960_materialize | 1.019x | 1.002x | 1.530x |
| chess960_perft_d3 | 0.986x | 0.985x | 1.492x |
| polyglot_key | 1.009x | 0.972x | 35.821x |
| polyglot_recompute | 0.978x | 0.977x | 0.961x |
| repetition_key | 0.853x | 0.815x | 23.696x |
| corpus_count | 0.979x | 0.968x | 1.149x |
| corpus_materialize | 0.975x | 0.969x | 1.136x |
| corpus_uci_sorted | 0.970x | 0.961x | 1.642x |
| replay_trusted_native | 1.003x | 0.981x | 0.705x |
| replay_checked_native | 0.995x | 0.976x | 1.083x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).

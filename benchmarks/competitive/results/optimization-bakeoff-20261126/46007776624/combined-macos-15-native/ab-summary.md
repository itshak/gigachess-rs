| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.979x | 1.586x |
| materialize | 1.003x | 0.976x | 1.111x |
| perft_bulk_d4 | 1.018x | 0.943x | 1.297x |
| perft_full_d3 | 1.119x | 0.997x | 1.479x |
| fen_parse | 0.958x | 0.902x | 1.260x |
| fen_write_native_ep | 0.974x | 0.923x | 1.737x |
| snapshot | 0.976x | 0.948x | 0.980x |
| san_parse_all | 0.929x | 0.794x | 1.649x |
| san_render_native | 0.988x | 0.975x | 2.907x |
| apply_children_full | 0.992x | 0.956x | 1.151x |
| make_unmake_full | 0.990x | 0.975x | 1.316x |
| startpos | 0.981x | 0.981x | 1.204x |
| chess960_materialize | 0.992x | 0.989x | 1.547x |
| chess960_perft_d3 | 1.024x | 1.019x | 1.576x |
| polyglot_key | 0.958x | 0.939x | 38.259x |
| polyglot_recompute | 0.910x | 0.901x | 0.939x |
| repetition_key | 0.995x | 0.979x | 19.769x |
| corpus_count | 1.121x | 1.108x | 1.326x |
| corpus_materialize | 1.000x | 0.989x | 1.178x |
| corpus_uci_sorted | 1.005x | 1.003x | 1.448x |
| replay_trusted_native | 1.010x | 0.983x | 0.656x |
| replay_checked_native | 0.997x | 0.971x | 1.280x |

Cold first legal move list: 0.003 ms -> 0.004 ms (0.72x).

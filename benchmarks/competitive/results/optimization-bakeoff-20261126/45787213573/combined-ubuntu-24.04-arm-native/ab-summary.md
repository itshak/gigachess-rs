| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.355x |
| materialize | 0.999x | 0.997x | 1.251x |
| perft_bulk_d4 | 1.001x | 0.999x | 1.355x |
| perft_full_d3 | 0.966x | 0.949x | 1.014x |
| fen_parse | 0.995x | 0.991x | 1.243x |
| fen_write_native_ep | 1.001x | 0.997x | 1.561x |
| snapshot | 0.997x | 0.988x | 0.893x |
| san_parse_all | 0.995x | 0.988x | 1.823x |
| san_render_native | 0.982x | 0.967x | 2.495x |
| apply_children_full | 1.004x | 0.959x | 1.025x |
| make_unmake_full | 0.937x | 0.877x | 1.347x |
| startpos | 1.000x | 1.000x | 1.187x |
| chess960_materialize | 0.951x | 0.922x | 1.260x |
| chess960_perft_d3 | 0.956x | 0.928x | 1.270x |
| polyglot_key | 1.013x | 1.009x | 25.656x |
| polyglot_recompute | 0.996x | 0.995x | 1.071x |
| repetition_key | 1.002x | 1.002x | 18.882x |
| corpus_count | 0.985x | 0.970x | 1.455x |
| corpus_materialize | 0.993x | 0.988x | 1.220x |
| corpus_uci_sorted | 1.003x | 1.002x | 1.451x |
| replay_trusted_native | 0.896x | 0.860x | 0.445x |
| replay_checked_native | 0.979x | 0.970x | 0.966x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.95x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.041x | 0.944x | 1.374x |
| materialize | 1.002x | 0.990x | 1.343x |
| perft_bulk_d4 | 1.087x | 1.068x | 1.383x |
| perft_full_d3 | 1.183x | 1.025x | 1.519x |
| fen_parse | 0.888x | 0.847x | 1.091x |
| fen_write_native_ep | 0.964x | 0.932x | 1.688x |
| snapshot | 0.999x | 0.997x | 1.013x |
| san_parse_all | 1.006x | 0.986x | 2.046x |
| san_render_native | 1.020x | 0.995x | 2.269x |
| apply_children_full | 0.999x | 0.958x | 1.358x |
| make_unmake_full | 1.015x | 0.999x | 1.401x |
| startpos | 1.000x | 1.000x | 1.319x |
| chess960_materialize | 1.005x | 0.989x | 1.456x |
| chess960_perft_d3 | 1.024x | 1.003x | 1.571x |
| polyglot_key | 1.002x | 0.999x | 23.802x |
| polyglot_recompute | 0.940x | 0.840x | 0.942x |
| repetition_key | 1.001x | 0.999x | 18.450x |
| corpus_count | 1.105x | 1.102x | 1.423x |
| corpus_materialize | 1.013x | 1.012x | 1.195x |
| corpus_uci_sorted | 1.014x | 1.009x | 1.829x |
| replay_trusted_native | 0.993x | 0.992x | 0.907x |
| replay_checked_native | 0.996x | 0.985x | 1.147x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.06x).

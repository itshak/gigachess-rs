| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 0.982x | 1.569x |
| materialize | 1.019x | 0.999x | 1.304x |
| perft_bulk_d4 | 1.039x | 1.001x | 1.447x |
| perft_full_d3 | 1.226x | 1.030x | 1.563x |
| fen_parse | 1.023x | 0.943x | 1.180x |
| fen_write_native_ep | 0.993x | 0.978x | 1.685x |
| snapshot | 1.002x | 1.000x | 1.249x |
| san_parse_all | 0.979x | 0.953x | 2.003x |
| san_render_native | 0.992x | 0.979x | 2.246x |
| apply_children_full | 1.013x | 0.997x | 1.324x |
| make_unmake_full | 1.019x | 0.958x | 1.301x |
| startpos | 1.000x | 1.000x | 1.250x |
| chess960_materialize | 1.011x | 1.002x | 1.388x |
| chess960_perft_d3 | 1.020x | 1.014x | 1.518x |
| polyglot_key | 1.003x | 1.002x | 29.976x |
| polyglot_recompute | 1.277x | 1.172x | 1.319x |
| repetition_key | 1.000x | 0.997x | 21.272x |
| corpus_count | 1.071x | 1.044x | 1.524x |
| corpus_materialize | 0.974x | 0.965x | 1.186x |
| corpus_uci_sorted | 0.999x | 0.998x | 1.769x |
| replay_trusted_native | 0.959x | 0.958x | 0.879x |
| replay_checked_native | 0.963x | 0.955x | 1.085x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.87x).

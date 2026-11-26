| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.979x | 1.120x |
| materialize | 0.949x | 0.926x | 1.229x |
| perft_bulk_d4 | 0.998x | 0.983x | 1.127x |
| perft_full_d3 | 0.992x | 0.979x | 0.743x |
| fen_parse | 1.065x | 1.046x | 1.321x |
| fen_write_native_ep | 0.995x | 0.974x | 1.164x |
| snapshot | 0.994x | 0.918x | 1.024x |
| san_parse_all | 1.008x | 0.990x | 1.632x |
| san_render_native | 0.994x | 0.952x | 2.180x |
| apply_children_full | 1.002x | 0.965x | 0.809x |
| make_unmake_full | 0.995x | 0.964x | 1.024x |
| startpos | 0.990x | 0.990x | 1.282x |
| chess960_materialize | 0.988x | 0.987x | 1.329x |
| chess960_perft_d3 | 1.003x | 0.998x | 1.265x |
| polyglot_key | 1.019x | 1.014x | 32.837x |
| polyglot_recompute | 0.986x | 0.976x | 1.002x |
| repetition_key | 1.215x | 1.134x | 24.943x |
| corpus_count | 0.985x | 0.981x | 1.074x |
| corpus_materialize | 0.944x | 0.895x | 1.257x |
| corpus_uci_sorted | 0.994x | 0.984x | 0.981x |
| replay_trusted_native | 0.987x | 0.977x | 0.509x |
| replay_checked_native | 0.954x | 0.917x | 0.981x |

Cold first legal move list: 0.016 ms -> 0.016 ms (1.00x).

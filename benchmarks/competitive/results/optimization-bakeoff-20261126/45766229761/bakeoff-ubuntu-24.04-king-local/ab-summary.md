| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.020x | 0.801x | 1.210x |
| materialize | 1.011x | 0.918x | 1.401x |
| perft_bulk_d4 | 0.997x | 0.889x | 0.868x |
| perft_full_d3 | 0.993x | 0.975x | 0.596x |
| fen_parse | 1.020x | 1.003x | 1.202x |
| fen_write_native_ep | 0.984x | 0.963x | 1.301x |
| snapshot | 1.321x | 1.309x | 0.787x |
| san_parse_all | 0.995x | 0.986x | 0.899x |
| san_render_native | 0.983x | 0.956x | 1.050x |
| apply_children_full | 1.002x | 0.965x | 0.644x |
| make_unmake_full | 0.990x | 0.975x | 0.753x |
| startpos | 1.062x | 1.062x | 0.662x |
| chess960_materialize | 1.007x | 1.000x | 1.171x |
| chess960_perft_d3 | 0.994x | 0.991x | 0.943x |
| polyglot_key | 0.922x | 0.884x | 35.046x |
| polyglot_recompute | 0.984x | 0.981x | 0.985x |
| repetition_key | 1.157x | 1.035x | 16.490x |
| corpus_count | 0.878x | 0.812x | 1.007x |
| corpus_materialize | 0.965x | 0.963x | 1.034x |
| corpus_uci_sorted | 0.996x | 0.996x | 0.616x |
| replay_trusted_native | 0.972x | 0.947x | 0.494x |
| replay_checked_native | 0.986x | 0.978x | 0.794x |

Cold first legal move list: 135.432 ms → 139.433 ms (0.97x).

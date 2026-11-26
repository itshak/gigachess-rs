| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.993x | 1.147x |
| materialize | 1.004x | 0.992x | 1.548x |
| perft_bulk_d4 | 0.919x | 0.882x | 1.046x |
| perft_full_d3 | 0.850x | 0.836x | 0.813x |
| fen_parse | 0.869x | 0.846x | 1.029x |
| fen_write_native_ep | 1.008x | 0.978x | 1.311x |
| snapshot | 0.817x | 0.814x | 0.815x |
| san_parse_all | 0.928x | 0.899x | 1.581x |
| san_render_native | 0.937x | 0.893x | 1.978x |
| apply_children_full | 0.876x | 0.863x | 0.860x |
| make_unmake_full | 0.809x | 0.772x | 0.789x |
| startpos | 0.778x | 0.778x | 1.055x |
| chess960_materialize | 0.995x | 0.990x | 1.374x |
| chess960_perft_d3 | 0.935x | 0.926x | 1.204x |
| polyglot_key | 0.866x | 0.753x | 36.638x |
| polyglot_recompute | 1.032x | 1.003x | 1.047x |
| repetition_key | 0.898x | 0.800x | 20.604x |
| corpus_count | 0.992x | 0.985x | 1.205x |
| corpus_materialize | 1.003x | 0.992x | 1.153x |
| corpus_uci_sorted | 1.005x | 1.002x | 0.971x |
| replay_trusted_native | 0.892x | 0.876x | 0.533x |
| replay_checked_native | 0.817x | 0.754x | 0.782x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.11x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 0.983x | 1.537x |
| materialize | 1.015x | 0.994x | 1.294x |
| perft_bulk_d4 | 1.028x | 0.978x | 1.446x |
| perft_full_d3 | 1.236x | 1.031x | 1.591x |
| fen_parse | 0.866x | 0.832x | 1.072x |
| fen_write_native_ep | 0.969x | 0.899x | 1.664x |
| snapshot | 0.999x | 0.994x | 1.250x |
| san_parse_all | 1.036x | 1.018x | 2.125x |
| san_render_native | 0.999x | 0.983x | 2.195x |
| apply_children_full | 1.007x | 0.997x | 1.331x |
| make_unmake_full | 1.009x | 0.957x | 1.292x |
| startpos | 1.000x | 1.000x | 1.250x |
| chess960_materialize | 0.990x | 0.957x | 1.411x |
| chess960_perft_d3 | 1.005x | 0.985x | 1.510x |
| polyglot_key | 0.994x | 0.948x | 25.802x |
| polyglot_recompute | 0.993x | 0.988x | 1.028x |
| repetition_key | 0.956x | 0.915x | 19.576x |
| corpus_count | 1.060x | 1.036x | 1.528x |
| corpus_materialize | 0.987x | 0.978x | 1.180x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.754x |
| replay_trusted_native | 0.972x | 0.969x | 0.878x |
| replay_checked_native | 0.970x | 0.958x | 1.085x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.11x).

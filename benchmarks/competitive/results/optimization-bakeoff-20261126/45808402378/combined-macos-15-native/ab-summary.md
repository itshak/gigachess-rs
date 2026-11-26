| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.912x | 0.693x | 1.344x |
| materialize | 1.084x | 0.826x | 0.997x |
| perft_bulk_d4 | 0.816x | 0.615x | 1.142x |
| perft_full_d3 | 1.007x | 0.851x | 1.333x |
| fen_parse | 0.961x | 0.856x | 1.148x |
| fen_write_native_ep | 1.041x | 0.914x | 1.930x |
| snapshot | 0.869x | 0.611x | 0.776x |
| san_parse_all | 0.964x | 0.886x | 1.890x |
| san_render_native | 0.990x | 0.909x | 2.650x |
| apply_children_full | 1.007x | 0.814x | 0.972x |
| make_unmake_full | 1.035x | 0.893x | 1.175x |
| startpos | 0.977x | 0.977x | 1.379x |
| chess960_materialize | 1.019x | 0.949x | 1.524x |
| chess960_perft_d3 | 0.821x | 0.754x | 1.210x |
| polyglot_key | 1.136x | 1.118x | 43.383x |
| polyglot_recompute | 1.023x | 0.920x | 1.123x |
| repetition_key | 1.054x | 0.980x | 18.758x |
| corpus_count | 0.770x | 0.746x | 0.963x |
| corpus_materialize | 1.151x | 1.097x | 1.349x |
| corpus_uci_sorted | 0.998x | 0.842x | 1.224x |
| replay_trusted_native | 1.027x | 0.982x | 0.501x |
| replay_checked_native | 0.930x | 0.822x | 0.995x |

Cold first legal move list: 0.014 ms -> 0.006 ms (2.16x).

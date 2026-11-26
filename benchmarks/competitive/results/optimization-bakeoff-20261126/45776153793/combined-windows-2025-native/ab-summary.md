| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.029x | 0.992x | 1.296x |
| materialize | 0.962x | 0.908x | 1.164x |
| perft_bulk_d4 | 0.902x | 0.823x | 1.210x |
| perft_full_d3 | 0.904x | 0.869x | 0.667x |
| fen_parse | 0.964x | 0.925x | 1.119x |
| fen_write_native_ep | 0.978x | 0.932x | 1.014x |
| snapshot | 0.860x | 0.824x | 0.891x |
| san_parse_all | 0.862x | 0.787x | 1.597x |
| san_render_native | 0.906x | 0.840x | 3.296x |
| apply_children_full | 0.887x | 0.819x | 0.735x |
| make_unmake_full | 0.834x | 0.656x | 0.828x |
| startpos | 0.431x | 0.431x | 0.479x |
| chess960_materialize | 0.934x | 0.893x | 1.189x |
| chess960_perft_d3 | 0.870x | 0.865x | 1.184x |
| polyglot_key | 0.972x | 0.946x | 29.693x |
| polyglot_recompute | 0.977x | 0.960x | 1.042x |
| repetition_key | 0.882x | 0.867x | 18.032x |
| corpus_count | 0.978x | 0.966x | 1.342x |
| corpus_materialize | 0.909x | 0.908x | 1.123x |
| corpus_uci_sorted | 0.958x | 0.930x | 1.002x |
| replay_trusted_native | 0.898x | 0.881x | 0.471x |
| replay_checked_native | 0.794x | 0.742x | 0.746x |

Cold first legal move list: 0.034 ms -> 0.037 ms (0.92x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.965x | 1.374x |
| materialize | 0.998x | 0.991x | 1.247x |
| perft_bulk_d4 | 0.998x | 0.990x | 1.231x |
| perft_full_d3 | 0.919x | 0.878x | 0.985x |
| fen_parse | 1.040x | 1.020x | 1.273x |
| fen_write_native_ep | 0.996x | 0.992x | 1.576x |
| snapshot | 1.111x | 1.105x | 0.988x |
| san_parse_all | 0.969x | 0.958x | 1.762x |
| san_render_native | 0.988x | 0.970x | 2.508x |
| apply_children_full | 0.856x | 0.827x | 1.096x |
| make_unmake_full | 1.010x | 0.960x | 1.529x |
| startpos | 0.832x | 0.832x | 1.023x |
| chess960_materialize | 0.958x | 0.912x | 1.321x |
| chess960_perft_d3 | 0.931x | 0.900x | 1.263x |
| polyglot_key | 1.038x | 1.023x | 26.353x |
| polyglot_recompute | 1.014x | 1.012x | 1.086x |
| repetition_key | 1.103x | 1.086x | 20.342x |
| corpus_count | 1.002x | 0.998x | 1.219x |
| corpus_materialize | 0.981x | 0.978x | 1.150x |
| corpus_uci_sorted | 1.000x | 0.995x | 1.451x |
| replay_trusted_native | 0.964x | 0.951x | 0.589x |
| replay_checked_native | 0.934x | 0.923x | 0.989x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.97x).

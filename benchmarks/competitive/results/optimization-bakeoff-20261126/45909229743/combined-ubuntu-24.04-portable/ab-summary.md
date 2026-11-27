| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.058x | 0.995x | 1.368x |
| materialize | 1.007x | 0.984x | 1.378x |
| perft_bulk_d4 | 1.033x | 0.985x | 1.322x |
| perft_full_d3 | 1.235x | 1.024x | 1.597x |
| fen_parse | 0.930x | 0.900x | 1.165x |
| fen_write_native_ep | 0.995x | 0.976x | 1.750x |
| snapshot | 0.997x | 0.992x | 0.976x |
| san_parse_all | 0.978x | 0.969x | 2.142x |
| san_render_native | 1.031x | 0.986x | 2.308x |
| apply_children_full | 0.995x | 0.972x | 1.401x |
| make_unmake_full | 0.989x | 0.957x | 1.375x |
| startpos | 1.000x | 1.000x | 1.335x |
| chess960_materialize | 1.032x | 1.027x | 1.687x |
| chess960_perft_d3 | 0.985x | 0.954x | 1.568x |
| polyglot_key | 1.004x | 1.001x | 22.323x |
| polyglot_recompute | 1.005x | 1.001x | 1.005x |
| repetition_key | 1.001x | 0.994x | 20.367x |
| corpus_count | 1.076x | 1.071x | 1.325x |
| corpus_materialize | 1.009x | 1.000x | 1.391x |
| corpus_uci_sorted | 1.004x | 0.997x | 1.605x |
| replay_trusted_native | 0.990x | 0.982x | 1.084x |
| replay_checked_native | 0.998x | 0.995x | 1.150x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.16x).

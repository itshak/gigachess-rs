| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.985x | 0.924x | 1.386x |
| materialize | 0.986x | 0.905x | 1.257x |
| perft_bulk_d4 | 0.980x | 0.884x | 1.380x |
| perft_full_d3 | 0.982x | 0.959x | 1.542x |
| fen_parse | 0.931x | 0.845x | 1.285x |
| fen_write_native_ep | 0.976x | 0.909x | 1.674x |
| snapshot | 0.699x | 0.670x | 1.005x |
| san_parse_all | 0.995x | 0.918x | 1.968x |
| san_render_native | 0.840x | 0.713x | 3.588x |
| apply_children_full | 0.982x | 0.907x | 1.032x |
| make_unmake_full | 0.980x | 0.926x | 1.238x |
| startpos | 0.999x | 0.999x | 1.342x |
| chess960_materialize | 1.024x | 0.991x | 1.139x |
| chess960_perft_d3 | 0.964x | 0.912x | 1.268x |
| polyglot_key | 0.915x | 0.912x | 33.843x |
| polyglot_recompute | 0.924x | 0.905x | 1.073x |
| repetition_key | 0.909x | 0.872x | 14.451x |
| corpus_count | 0.967x | 0.963x | 1.447x |
| corpus_materialize | 1.012x | 1.008x | 1.165x |
| corpus_uci_sorted | 1.065x | 1.043x | 1.491x |
| replay_trusted_native | 0.964x | 0.923x | 0.646x |
| replay_checked_native | 0.900x | 0.802x | 0.885x |

Cold first legal move list: 0.031 ms -> 0.031 ms (1.02x).

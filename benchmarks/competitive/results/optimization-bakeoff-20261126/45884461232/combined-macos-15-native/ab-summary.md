| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.830x | 0.541x | 1.276x |
| materialize | 1.087x | 0.979x | 1.241x |
| perft_bulk_d4 | 1.076x | 0.901x | 1.419x |
| perft_full_d3 | 0.953x | 0.757x | 1.264x |
| fen_parse | 1.102x | 0.741x | 1.301x |
| fen_write_native_ep | 0.960x | 0.804x | 1.617x |
| snapshot | 0.942x | 0.685x | 0.970x |
| san_parse_all | 0.939x | 0.701x | 1.803x |
| san_render_native | 0.918x | 0.646x | 2.782x |
| apply_children_full | 1.012x | 0.781x | 1.075x |
| make_unmake_full | 0.938x | 0.790x | 1.209x |
| startpos | 0.794x | 0.794x | 0.856x |
| chess960_materialize | 1.164x | 0.941x | 1.549x |
| chess960_perft_d3 | 0.953x | 0.881x | 1.514x |
| polyglot_key | 0.870x | 0.807x | 40.667x |
| polyglot_recompute | 0.792x | 0.659x | 0.983x |
| repetition_key | 1.107x | 1.096x | 19.556x |
| corpus_count | 1.027x | 0.766x | 1.144x |
| corpus_materialize | 0.986x | 0.969x | 1.110x |
| corpus_uci_sorted | 0.892x | 0.832x | 1.523x |
| replay_trusted_native | 0.775x | 0.653x | 0.560x |
| replay_checked_native | 0.814x | 0.681x | 1.204x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.90x).

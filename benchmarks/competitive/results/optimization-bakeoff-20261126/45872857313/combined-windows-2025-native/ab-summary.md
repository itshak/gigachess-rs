| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.898x | 0.732x | 1.499x |
| materialize | 0.986x | 0.937x | 1.418x |
| perft_bulk_d4 | 0.893x | 0.733x | 1.111x |
| perft_full_d3 | 0.932x | 0.760x | 1.339x |
| fen_parse | 0.921x | 0.830x | 1.208x |
| fen_write_native_ep | 0.947x | 0.780x | 1.727x |
| snapshot | 0.900x | 0.738x | 1.059x |
| san_parse_all | 0.906x | 0.804x | 1.928x |
| san_render_native | 0.884x | 0.616x | 4.184x |
| apply_children_full | 0.898x | 0.793x | 1.258x |
| make_unmake_full | 0.900x | 0.711x | 1.185x |
| startpos | 1.009x | 1.009x | 1.002x |
| chess960_materialize | 1.023x | 1.006x | 1.352x |
| chess960_perft_d3 | 0.991x | 0.980x | 1.485x |
| polyglot_key | 0.997x | 0.994x | 40.825x |
| polyglot_recompute | 1.002x | 0.995x | 1.021x |
| repetition_key | 1.276x | 1.276x | 33.778x |
| corpus_count | 0.980x | 0.971x | 0.932x |
| corpus_materialize | 1.019x | 1.004x | 1.078x |
| corpus_uci_sorted | 1.021x | 0.992x | 1.539x |
| replay_trusted_native | 1.104x | 1.060x | 0.872x |
| replay_checked_native | 1.012x | 0.989x | 1.196x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.93x).

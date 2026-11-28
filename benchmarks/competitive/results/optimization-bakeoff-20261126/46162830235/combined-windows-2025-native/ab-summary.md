| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.866x | 1.459x |
| materialize | 0.997x | 0.957x | 1.160x |
| perft_bulk_d4 | 1.049x | 1.008x | 1.447x |
| perft_full_d3 | 1.104x | 0.810x | 1.415x |
| fen_parse | 1.036x | 0.939x | 1.271x |
| fen_write_native_ep | 0.985x | 0.885x | 1.506x |
| snapshot | 0.991x | 0.932x | 1.480x |
| san_parse_all | 0.998x | 0.978x | 2.012x |
| san_render_native | 0.992x | 0.976x | 4.115x |
| apply_children_full | 0.826x | 0.766x | 1.134x |
| make_unmake_full | 1.030x | 0.976x | 1.298x |
| startpos | 1.024x | 1.024x | 0.996x |
| chess960_materialize | 1.020x | 0.996x | 1.413x |
| chess960_perft_d3 | 1.019x | 1.013x | 1.495x |
| polyglot_key | 0.864x | 0.751x | 31.441x |
| polyglot_recompute | 1.558x | 1.337x | 1.523x |
| repetition_key | 0.997x | 0.994x | 27.807x |
| corpus_count | 1.076x | 1.035x | 1.234x |
| corpus_materialize | 1.009x | 0.994x | 1.064x |
| corpus_uci_sorted | 0.953x | 0.900x | 1.296x |
| replay_trusted_native | 1.008x | 0.991x | 0.732x |
| replay_checked_native | 0.986x | 0.976x | 1.155x |

Cold first legal move list: 0.015 ms -> 0.015 ms (0.99x).

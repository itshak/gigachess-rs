| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.804x | 1.304x |
| materialize | 0.939x | 0.795x | 1.023x |
| perft_bulk_d4 | 0.916x | 0.680x | 1.101x |
| perft_full_d3 | 1.041x | 0.878x | 1.435x |
| fen_parse | 0.997x | 0.829x | 1.219x |
| fen_write_native_ep | 0.958x | 0.778x | 1.680x |
| snapshot | 0.988x | 0.778x | 0.949x |
| san_parse_all | 0.969x | 0.767x | 1.849x |
| san_render_native | 0.986x | 0.899x | 2.814x |
| apply_children_full | 0.962x | 0.795x | 1.119x |
| make_unmake_full | 0.928x | 0.753x | 1.310x |
| startpos | 0.480x | 0.480x | 0.580x |
| chess960_materialize | 0.975x | 0.914x | 1.271x |
| chess960_perft_d3 | 1.277x | 1.247x | 1.244x |
| polyglot_key | 1.120x | 0.934x | 38.592x |
| polyglot_recompute | 0.985x | 0.983x | 1.017x |
| repetition_key | 0.900x | 0.888x | 18.370x |
| corpus_count | 0.891x | 0.889x | 1.210x |
| corpus_materialize | 0.893x | 0.822x | 1.053x |
| corpus_uci_sorted | 1.036x | 0.951x | 1.462x |
| replay_trusted_native | 0.993x | 0.881x | 0.672x |
| replay_checked_native | 0.829x | 0.720x | 1.135x |

Cold first legal move list: 0.003 ms -> 0.006 ms (0.60x).

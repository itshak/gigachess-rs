| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.971x | 1.468x |
| materialize | 1.051x | 1.002x | 1.189x |
| perft_bulk_d4 | 1.004x | 0.925x | 1.349x |
| perft_full_d3 | 0.887x | 0.840x | 1.132x |
| fen_parse | 0.936x | 0.877x | 1.255x |
| fen_write_native_ep | 1.003x | 0.948x | 1.501x |
| snapshot | 1.012x | 0.981x | 1.500x |
| san_parse_all | 0.961x | 0.930x | 1.862x |
| san_render_native | 0.848x | 0.817x | 3.258x |
| apply_children_full | 0.999x | 0.954x | 1.294x |
| make_unmake_full | 0.955x | 0.830x | 1.253x |
| startpos | 1.235x | 1.235x | 1.246x |
| chess960_materialize | 0.913x | 0.843x | 1.315x |
| chess960_perft_d3 | 0.921x | 0.887x | 1.443x |
| polyglot_key | 0.928x | 0.921x | 32.694x |
| polyglot_recompute | 1.008x | 0.977x | 0.996x |
| repetition_key | 0.831x | 0.777x | 23.490x |
| corpus_count | 0.947x | 0.946x | 1.134x |
| corpus_materialize | 0.975x | 0.974x | 1.075x |
| corpus_uci_sorted | 0.952x | 0.937x | 1.326x |
| replay_trusted_native | 1.035x | 1.034x | 0.708x |
| replay_checked_native | 0.942x | 0.882x | 1.037x |

Cold first legal move list: 0.010 ms -> 0.015 ms (0.67x).

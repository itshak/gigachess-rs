| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.937x | 1.357x |
| materialize | 1.008x | 0.956x | 1.202x |
| perft_bulk_d4 | 0.978x | 0.923x | 1.218x |
| perft_full_d3 | 1.006x | 0.984x | 1.037x |
| fen_parse | 0.971x | 0.922x | 1.008x |
| fen_write_native_ep | 0.995x | 0.974x | 1.858x |
| snapshot | 1.228x | 1.214x | 1.248x |
| san_parse_all | 1.031x | 1.026x | 2.225x |
| san_render_native | 1.001x | 0.965x | 2.131x |
| apply_children_full | 1.499x | 1.325x | 1.150x |
| make_unmake_full | 1.044x | 0.983x | 1.227x |
| startpos | 1.095x | 1.095x | 1.200x |
| chess960_materialize | 1.080x | 0.992x | 1.464x |
| chess960_perft_d3 | 1.057x | 0.975x | 1.466x |
| polyglot_key | 0.991x | 0.983x | 20.500x |
| polyglot_recompute | 1.095x | 1.025x | 1.131x |
| repetition_key | 0.867x | 0.854x | 15.549x |
| corpus_count | 0.896x | 0.808x | 1.189x |
| corpus_materialize | 0.990x | 0.969x | 1.304x |
| corpus_uci_sorted | 0.999x | 0.984x | 1.684x |
| replay_trusted_native | 1.109x | 1.103x | 0.873x |
| replay_checked_native | 1.027x | 1.021x | 1.008x |

Cold first legal move list: 0.012 ms -> 0.006 ms (1.92x).

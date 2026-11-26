| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.964x | 0.784x | 1.380x |
| materialize | 0.923x | 0.705x | 0.978x |
| perft_bulk_d4 | 0.988x | 0.823x | 1.269x |
| perft_full_d3 | 1.018x | 0.803x | 1.358x |
| fen_parse | 0.985x | 0.803x | 1.179x |
| fen_write_native_ep | 0.995x | 0.933x | 1.793x |
| snapshot | 0.977x | 0.463x | 0.895x |
| san_parse_all | 0.945x | 0.688x | 1.919x |
| san_render_native | 0.881x | 0.740x | 2.496x |
| apply_children_full | 0.918x | 0.764x | 1.040x |
| make_unmake_full | 0.904x | 0.730x | 1.211x |
| startpos | 0.975x | 0.975x | 1.826x |
| chess960_materialize | 1.314x | 1.191x | 1.405x |
| chess960_perft_d3 | 1.171x | 1.062x | 1.461x |
| polyglot_key | 1.181x | 1.034x | 38.787x |
| polyglot_recompute | 1.020x | 0.874x | 0.995x |
| repetition_key | 1.021x | 0.992x | 15.100x |
| corpus_count | 1.017x | 0.994x | 1.610x |
| corpus_materialize | 1.236x | 1.125x | 1.164x |
| corpus_uci_sorted | 0.914x | 0.779x | 1.301x |
| replay_trusted_native | 1.201x | 1.002x | 0.571x |
| replay_checked_native | 1.154x | 1.129x | 1.023x |

Cold first legal move list: 0.015 ms -> 0.017 ms (0.88x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.095x | 0.837x | 1.571x |
| materialize | 1.039x | 0.961x | 1.049x |
| perft_bulk_d4 | 1.156x | 0.922x | 1.314x |
| perft_full_d3 | 1.227x | 0.975x | 1.476x |
| fen_parse | 1.045x | 0.835x | 1.226x |
| fen_write_native_ep | 1.096x | 0.914x | 1.714x |
| snapshot | 0.984x | 0.779x | 0.888x |
| san_parse_all | 1.058x | 0.908x | 1.665x |
| san_render_native | 1.018x | 0.818x | 2.872x |
| apply_children_full | 1.076x | 0.928x | 1.112x |
| make_unmake_full | 1.086x | 0.870x | 1.373x |
| startpos | 1.055x | 1.055x | 1.611x |
| chess960_materialize | 0.655x | 0.538x | 1.148x |
| chess960_perft_d3 | 0.881x | 0.852x | 1.684x |
| polyglot_key | 1.036x | 0.930x | 43.534x |
| polyglot_recompute | 1.457x | 1.231x | 1.428x |
| repetition_key | 0.878x | 0.804x | 19.677x |
| corpus_count | 0.900x | 0.724x | 1.187x |
| corpus_materialize | 1.000x | 0.893x | 1.211x |
| corpus_uci_sorted | 0.680x | 0.478x | 1.096x |
| replay_trusted_native | 0.980x | 0.823x | 0.617x |
| replay_checked_native | 1.102x | 1.085x | 1.449x |

Cold first legal move list: 0.005 ms -> 0.008 ms (0.73x).

| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.947x | 1.290x |
| materialize | 0.997x | 0.955x | 1.041x |
| perft_bulk_d4 | 0.985x | 0.955x | 1.315x |
| perft_full_d3 | 0.995x | 0.898x | 1.345x |
| fen_parse | 1.025x | 0.985x | 1.273x |
| fen_write_native_ep | 0.991x | 0.976x | 1.308x |
| snapshot | 0.988x | 0.905x | 0.977x |
| san_parse_all | 0.989x | 0.931x | 1.825x |
| san_render_native | 0.986x | 0.957x | 2.650x |
| apply_children_full | 0.972x | 0.954x | 0.989x |
| make_unmake_full | 0.967x | 0.916x | 1.210x |
| startpos | 0.979x | 0.979x | 1.209x |
| chess960_materialize | 0.932x | 0.899x | 1.296x |
| chess960_perft_d3 | 0.976x | 0.961x | 1.508x |
| polyglot_key | 1.003x | 0.999x | 39.594x |
| polyglot_recompute | 1.022x | 0.999x | 1.090x |
| repetition_key | 0.973x | 0.967x | 19.680x |
| corpus_count | 0.969x | 0.966x | 1.428x |
| corpus_materialize | 0.979x | 0.961x | 1.157x |
| corpus_uci_sorted | 0.921x | 0.907x | 1.463x |
| replay_trusted_native | 0.901x | 0.855x | 0.580x |
| replay_checked_native | 0.995x | 0.965x | 0.989x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.24x).

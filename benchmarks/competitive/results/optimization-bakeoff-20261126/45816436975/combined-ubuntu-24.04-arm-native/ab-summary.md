| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 0.941x | 1.379x |
| materialize | 1.005x | 0.959x | 1.245x |
| perft_bulk_d4 | 0.918x | 0.896x | 1.232x |
| perft_full_d3 | 0.983x | 0.966x | 1.038x |
| fen_parse | 1.003x | 0.994x | 1.240x |
| fen_write_native_ep | 0.995x | 0.987x | 1.568x |
| snapshot | 0.915x | 0.912x | 0.947x |
| san_parse_all | 1.011x | 0.999x | 1.838x |
| san_render_native | 0.997x | 0.978x | 2.549x |
| apply_children_full | 1.030x | 1.013x | 1.179x |
| make_unmake_full | 1.052x | 1.035x | 1.515x |
| startpos | 0.999x | 0.999x | 1.300x |
| chess960_materialize | 1.054x | 1.002x | 1.406x |
| chess960_perft_d3 | 1.034x | 0.987x | 1.358x |
| polyglot_key | 1.019x | 0.988x | 25.438x |
| polyglot_recompute | 1.038x | 1.028x | 1.115x |
| repetition_key | 0.962x | 0.932x | 18.396x |
| corpus_count | 0.824x | 0.792x | 1.205x |
| corpus_materialize | 0.976x | 0.966x | 1.158x |
| corpus_uci_sorted | 0.997x | 0.996x | 1.454x |
| replay_trusted_native | 1.055x | 1.039x | 0.549x |
| replay_checked_native | 1.038x | 1.036x | 1.028x |

Cold first legal move list: 0.011 ms -> 0.007 ms (1.59x).

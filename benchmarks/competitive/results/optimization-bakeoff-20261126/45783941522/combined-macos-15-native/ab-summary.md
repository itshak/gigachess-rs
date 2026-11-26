| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.837x | 0.612x | 1.355x |
| materialize | 0.863x | 0.696x | 0.957x |
| perft_bulk_d4 | 1.006x | 0.787x | 1.273x |
| perft_full_d3 | 0.997x | 0.867x | 1.462x |
| fen_parse | 1.041x | 0.838x | 1.271x |
| fen_write_native_ep | 1.023x | 0.879x | 1.691x |
| snapshot | 1.023x | 0.776x | 0.888x |
| san_parse_all | 0.917x | 0.761x | 1.818x |
| san_render_native | 0.913x | 0.807x | 2.792x |
| apply_children_full | 0.907x | 0.745x | 0.908x |
| make_unmake_full | 0.987x | 0.816x | 1.192x |
| startpos | 1.103x | 1.103x | 0.732x |
| chess960_materialize | 1.528x | 1.402x | 1.634x |
| chess960_perft_d3 | 1.360x | 1.229x | 1.396x |
| polyglot_key | 1.161x | 0.977x | 40.034x |
| polyglot_recompute | 1.354x | 1.297x | 1.002x |
| repetition_key | 1.160x | 1.086x | 19.729x |
| corpus_count | 1.168x | 1.050x | 1.347x |
| corpus_materialize | 1.297x | 1.032x | 1.203x |
| corpus_uci_sorted | 1.414x | 1.193x | 1.437x |
| replay_trusted_native | 1.145x | 0.965x | 0.577x |
| replay_checked_native | 1.272x | 1.205x | 1.005x |

Cold first legal move list: 0.022 ms -> 0.014 ms (1.57x).

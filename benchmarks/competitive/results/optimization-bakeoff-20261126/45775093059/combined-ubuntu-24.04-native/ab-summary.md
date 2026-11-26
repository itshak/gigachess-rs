| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.167x | 1.091x | 1.387x |
| materialize | 0.988x | 0.951x | 1.413x |
| perft_bulk_d4 | 1.339x | 1.272x | 1.168x |
| perft_full_d3 | 2.418x | 2.188x | 1.424x |
| fen_parse | 1.126x | 1.051x | 1.122x |
| fen_write_native_ep | 0.993x | 0.972x | 1.253x |
| snapshot | 1.665x | 1.652x | 1.149x |
| san_parse_all | 2.047x | 1.968x | 1.837x |
| san_render_native | 2.057x | 1.941x | 2.146x |
| apply_children_full | 1.554x | 1.522x | 0.916x |
| make_unmake_full | 1.366x | 1.271x | 0.986x |
| startpos | 1.800x | 1.800x | 1.117x |
| chess960_materialize | 1.101x | 0.993x | 1.319x |
| chess960_perft_d3 | 1.413x | 1.322x | 1.345x |
| polyglot_key | 1.098x | 1.005x | 39.270x |
| polyglot_recompute | 0.884x | 0.851x | 0.863x |
| repetition_key | 1.459x | 1.188x | 22.984x |
| corpus_count | 1.144x | 1.086x | 1.315x |
| corpus_materialize | 1.024x | 1.019x | 1.095x |
| corpus_uci_sorted | 1.494x | 1.487x | 0.938x |
| replay_trusted_native | 1.250x | 1.181x | 0.637x |
| replay_checked_native | 1.270x | 1.121x | 0.991x |

Cold first legal move list: 133.182 ms -> 0.009 ms (14317.55x).

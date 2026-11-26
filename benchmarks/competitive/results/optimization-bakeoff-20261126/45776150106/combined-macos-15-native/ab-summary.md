| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.938x | 0.799x | 1.176x |
| materialize | 1.035x | 0.769x | 1.112x |
| perft_bulk_d4 | 1.015x | 0.798x | 1.311x |
| perft_full_d3 | 0.951x | 0.788x | 0.724x |
| fen_parse | 0.978x | 0.789x | 1.167x |
| fen_write_native_ep | 0.963x | 0.822x | 1.316x |
| snapshot | 0.622x | 0.499x | 0.614x |
| san_parse_all | 0.980x | 0.888x | 1.700x |
| san_render_native | 0.959x | 0.796x | 2.789x |
| apply_children_full | 0.834x | 0.747x | 0.713x |
| make_unmake_full | 0.757x | 0.689x | 0.858x |
| startpos | 0.664x | 0.664x | 0.746x |
| chess960_materialize | 0.890x | 0.834x | 1.223x |
| chess960_perft_d3 | 0.721x | 0.684x | 1.199x |
| polyglot_key | 0.708x | 0.669x | 28.414x |
| polyglot_recompute | 0.878x | 0.777x | 0.903x |
| repetition_key | 0.724x | 0.665x | 15.857x |
| corpus_count | 0.922x | 0.889x | 1.431x |
| corpus_materialize | 1.111x | 1.023x | 1.165x |
| corpus_uci_sorted | 1.153x | 1.012x | 1.041x |
| replay_trusted_native | 0.972x | 0.832x | 0.441x |
| replay_checked_native | 0.782x | 0.718x | 0.796x |

Cold first legal move list: 0.013 ms -> 0.011 ms (1.21x).

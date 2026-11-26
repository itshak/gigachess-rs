| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.919x | 0.867x | 1.157x |
| materialize | 0.984x | 0.867x | 1.086x |
| perft_bulk_d4 | 1.103x | 1.021x | 1.107x |
| perft_full_d3 | 0.974x | 0.905x | 0.597x |
| fen_parse | 1.038x | 0.960x | 1.405x |
| fen_write_native_ep | 0.982x | 0.930x | 1.227x |
| snapshot | 1.586x | 1.458x | 1.130x |
| san_parse_all | 1.938x | 1.714x | 1.792x |
| san_render_native | 1.713x | 1.357x | 3.383x |
| apply_children_full | 1.164x | 1.050x | 0.731x |
| make_unmake_full | 1.059x | 0.987x | 1.016x |
| startpos | 1.501x | 1.501x | 1.250x |
| chess960_materialize | 1.296x | 1.031x | 1.188x |
| chess960_perft_d3 | 1.247x | 1.113x | 1.177x |
| polyglot_key | 1.215x | 0.921x | 36.265x |
| polyglot_recompute | 0.953x | 0.941x | 1.030x |
| repetition_key | 1.271x | 1.210x | 17.052x |
| corpus_count | 0.968x | 0.958x | 1.107x |
| corpus_materialize | 1.000x | 0.968x | 1.174x |
| corpus_uci_sorted | 1.115x | 1.105x | 0.934x |
| replay_trusted_native | 0.876x | 0.823x | 0.458x |
| replay_checked_native | 1.257x | 1.115x | 0.906x |
